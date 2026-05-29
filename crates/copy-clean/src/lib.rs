use std::path::Path;
use ttk_core::fs_utils::copy_clean_dir;

const SKIP_DIRS: &[&str] = &["node_modules", ".git", ".github", "dist"];
const SKIP_EXTS: &[&str] = &["md5", "sha1", "zip"];

pub struct CopyCleanArgs<'a> {
    pub source: &'a Path,
    pub dest: &'a Path,
}

pub fn run(args: CopyCleanArgs) -> Result<(), String> {
    if !args.source.exists() {
        return Err(format!("source not found: {}", args.source.display()));
    }
    if args.dest.exists() {
        return Err(format!("destination already exists: {}", args.dest.display()));
    }

    copy_clean_dir(args.source, args.dest, SKIP_DIRS, SKIP_EXTS)?;

    println!("copied {} → {}", args.source.display(), args.dest.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    fn write_file(path: &Path, content: &str) {
        let mut f = File::create(path).unwrap();
        f.write_all(content.as_bytes()).unwrap();
    }

    #[test]
    fn test_copy_clean_missing_source_err() {
        let tmp = tempdir().unwrap();
        let dest = tmp.path().join("out");
        let result = run(CopyCleanArgs {
            source: &tmp.path().join("nope"),
            dest: &dest,
        });
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("source not found"));
    }

    #[test]
    fn test_copy_clean_dest_exists_err() {
        let src = tempdir().unwrap();
        let dest = tempdir().unwrap();
        let result = run(CopyCleanArgs {
            source: src.path(),
            dest: dest.path(),
        });
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("destination already exists"));
    }

    #[test]
    fn test_copy_clean_skips_node_modules() {
        let src = tempdir().unwrap();
        let dest_parent = tempdir().unwrap();
        let dest = dest_parent.path().join("out");

        fs::create_dir(src.path().join("node_modules")).unwrap();
        write_file(&src.path().join("node_modules").join("pkg"), "x");
        write_file(&src.path().join("main.js"), "y");

        run(CopyCleanArgs {
            source: src.path(),
            dest: &dest,
        })
        .unwrap();

        assert!(dest.join("main.js").exists());
        assert!(!dest.join("node_modules").exists());
    }

    #[test]
    fn test_copy_clean_skips_checksum_files() {
        let src = tempdir().unwrap();
        let dest_parent = tempdir().unwrap();
        let dest = dest_parent.path().join("out");

        write_file(&src.path().join("out.md5"), "hash");
        write_file(&src.path().join("out.sha1"), "hash");
        write_file(&src.path().join("arch.zip"), "zip");
        write_file(&src.path().join("app.js"), "code");

        run(CopyCleanArgs {
            source: src.path(),
            dest: &dest,
        })
        .unwrap();

        assert!(!dest.join("out.md5").exists());
        assert!(!dest.join("out.sha1").exists());
        assert!(!dest.join("arch.zip").exists());
        assert!(dest.join("app.js").exists());
    }
}
