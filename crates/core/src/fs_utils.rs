use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub fn copy_clean_dir(src: &Path, dest: &Path, skip_dirs: &[&str], skip_exts: &[&str]) -> Result<(), String> {
    for entry in WalkDir::new(src).follow_links(false) {
        let entry = entry.map_err(|e| format!("walk error: {}", e))?;
        let path = entry.path();

        if path == src {
            fs::create_dir_all(dest)
                .map_err(|e| format!("cannot create dest {}: {}", dest.display(), e))?;
            continue;
        }

        let components: Vec<_> = path
            .strip_prefix(src)
            .map_err(|e| format!("strip prefix error: {}", e))?
            .components()
            .collect();

        let should_skip = components.iter().any(|c| {
            let name = c.as_os_str().to_string_lossy();
            skip_dirs.contains(&name.as_ref())
        });
        if should_skip {
            continue;
        }

        let rel = path.strip_prefix(src).unwrap();
        let target = dest.join(rel);

        if entry.path_is_symlink() {
            continue;
        }

        if path.is_dir() {
            fs::create_dir_all(&target)
                .map_err(|e| format!("cannot create dir {}: {}", target.display(), e))?;
        } else {
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                if skip_exts.contains(&ext_str.as_ref()) {
                    continue;
                }
            }
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("cannot create parent {}: {}", parent.display(), e))?;
            }
            fs::copy(path, &target)
                .map_err(|e| format!("copy {} → {}: {}", path.display(), target.display(), e))?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    fn write_file(path: &Path, content: &str) {
        let mut f = File::create(path).unwrap();
        f.write_all(content.as_bytes()).unwrap();
    }

    #[test]
    fn test_copy_clean_copies_files() {
        let src = tempdir().unwrap();
        let dest_parent = tempdir().unwrap();
        let dest = dest_parent.path().join("out");

        write_file(&src.path().join("a.txt"), "hello");
        write_file(&src.path().join("b.txt"), "world");

        copy_clean_dir(src.path(), &dest, &[], &[]).unwrap();

        assert!(dest.join("a.txt").exists());
        assert!(dest.join("b.txt").exists());
    }

    #[test]
    fn test_copy_clean_skips_dirs() {
        let src = tempdir().unwrap();
        let dest_parent = tempdir().unwrap();
        let dest = dest_parent.path().join("out");

        fs::create_dir(src.path().join("node_modules")).unwrap();
        write_file(&src.path().join("node_modules").join("pkg.js"), "x");
        write_file(&src.path().join("keep.txt"), "y");

        copy_clean_dir(src.path(), &dest, &["node_modules"], &[]).unwrap();

        assert!(dest.join("keep.txt").exists());
        assert!(!dest.join("node_modules").exists());
    }

    #[test]
    fn test_copy_clean_skips_extensions() {
        let src = tempdir().unwrap();
        let dest_parent = tempdir().unwrap();
        let dest = dest_parent.path().join("out");

        write_file(&src.path().join("file.md5"), "hash");
        write_file(&src.path().join("file.zip"), "zip");
        write_file(&src.path().join("main.rs"), "fn main() {}");

        copy_clean_dir(src.path(), &dest, &[], &["md5", "zip"]).unwrap();

        assert!(!dest.join("file.md5").exists());
        assert!(!dest.join("file.zip").exists());
        assert!(dest.join("main.rs").exists());
    }

    #[test]
    fn test_copy_clean_no_follow_symlinks() {
        let src = tempdir().unwrap();
        let dest_parent = tempdir().unwrap();
        let dest = dest_parent.path().join("out");

        write_file(&src.path().join("real.txt"), "real");

        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(src.path().join("real.txt"), src.path().join("link.txt"))
                .unwrap();
        }

        copy_clean_dir(src.path(), &dest, &[], &[]).unwrap();

        assert!(dest.join("real.txt").exists());
        #[cfg(unix)]
        assert!(!dest.join("link.txt").exists());
    }

    #[test]
    fn test_copy_clean_preserves_subdir_structure() {
        let src = tempdir().unwrap();
        let dest_parent = tempdir().unwrap();
        let dest = dest_parent.path().join("out");

        fs::create_dir(src.path().join("sub")).unwrap();
        write_file(&src.path().join("sub").join("nested.txt"), "n");

        copy_clean_dir(src.path(), &dest, &[], &[]).unwrap();

        assert!(dest.join("sub").join("nested.txt").exists());
    }
}
