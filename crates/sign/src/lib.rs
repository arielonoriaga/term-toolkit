use sha1::{Digest, Sha1};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub struct SignArgs<'a> {
    pub folder: &'a Path,
    pub prefix: &'a str,
}

pub fn sign_dir(folder: &Path, prefix: &str) -> Result<(), String> {
    let mut md5_lines: Vec<String> = Vec::new();
    let mut sha1_lines: Vec<String> = Vec::new();

    for entry in WalkDir::new(folder)
        .follow_links(false)
        .into_iter()
        .filter(|e| {
            e.as_ref().map(|e| {
                let ext = e.path().extension().and_then(|x| x.to_str()).unwrap_or("");
                ext != "md5" && ext != "sha1"
            }).unwrap_or(true)
        })
    {
        let entry = entry.map_err(|e| format!("walk error: {}", e))?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let bytes = fs::read(path)
            .map_err(|e| format!("read {}: {}", path.display(), e))?;

        let md5_hash = format!("{:x}", md5::compute(&bytes));
        let sha1_hash = {
            let mut h = Sha1::new();
            h.update(&bytes);
            format!("{:x}", h.finalize())
        };

        let rel = path.strip_prefix(folder)
            .map_err(|e| e.to_string())?;
        let display = rel.display().to_string();
        md5_lines.push(format!("{}  {}", md5_hash, display));
        sha1_lines.push(format!("{}  {}", sha1_hash, display));
    }

    md5_lines.sort();
    sha1_lines.sort();

    let md5_path = format!("{}.md5", prefix);
    let sha1_path = format!("{}.sha1", prefix);

    fs::write(&md5_path, md5_lines.join("\n") + "\n")
        .map_err(|e| format!("write {}: {}", md5_path, e))?;
    fs::write(&sha1_path, sha1_lines.join("\n") + "\n")
        .map_err(|e| format!("write {}: {}", sha1_path, e))?;

    println!("wrote {} and {}", md5_path, sha1_path);
    Ok(())
}

pub fn run(args: SignArgs) -> Result<(), String> {
    if !args.folder.exists() {
        return Err(format!("folder not found: {}", args.folder.display()));
    }
    sign_dir(args.folder, args.prefix)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    fn write_file(path: &Path, content: &[u8]) {
        let mut f = File::create(path).unwrap();
        f.write_all(content).unwrap();
    }

    #[test]
    fn test_sign_missing_folder_returns_err() {
        let tmp = tempdir().unwrap();
        let result = run(SignArgs {
            folder: &tmp.path().join("nope"),
            prefix: "/tmp/out",
        });
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("folder not found"));
    }

    #[test]
    fn test_sign_produces_md5_sha1_files() {
        let src = tempdir().unwrap();
        let out = tempdir().unwrap();
        let prefix = out.path().join("checksums").to_string_lossy().into_owned();

        write_file(&src.path().join("hello.txt"), b"hello");

        run(SignArgs {
            folder: src.path(),
            prefix: &prefix,
        })
        .unwrap();

        let md5_content = fs::read_to_string(format!("{}.md5", prefix)).unwrap();
        let sha1_content = fs::read_to_string(format!("{}.sha1", prefix)).unwrap();

        assert!(md5_content.contains("  "));
        assert!(sha1_content.contains("  "));
        assert!(md5_content.ends_with('\n'));
        assert!(sha1_content.ends_with('\n'));
    }

    #[test]
    fn test_sign_checksum_format_two_spaces() {
        let src = tempdir().unwrap();
        let out = tempdir().unwrap();
        let prefix = out.path().join("cs").to_string_lossy().into_owned();

        write_file(&src.path().join("a.bin"), b"abc");

        run(SignArgs {
            folder: src.path(),
            prefix: &prefix,
        })
        .unwrap();

        let md5_content = fs::read_to_string(format!("{}.md5", prefix)).unwrap();
        let line = md5_content.lines().next().unwrap();
        let parts: Vec<&str> = line.splitn(2, "  ").collect();
        assert_eq!(parts.len(), 2, "must have exactly two spaces between hash and path");
        assert_eq!(parts[0].len(), 32, "md5 hash must be 32 hex chars");
    }

    #[test]
    fn test_sign_sha1_hash_length() {
        let src = tempdir().unwrap();
        let out = tempdir().unwrap();
        let prefix = out.path().join("cs").to_string_lossy().into_owned();

        write_file(&src.path().join("b.bin"), b"data");

        run(SignArgs {
            folder: src.path(),
            prefix: &prefix,
        })
        .unwrap();

        let sha1_content = fs::read_to_string(format!("{}.sha1", prefix)).unwrap();
        let line = sha1_content.lines().next().unwrap();
        let hash = line.splitn(2, "  ").next().unwrap();
        assert_eq!(hash.len(), 40, "sha1 hash must be 40 hex chars");
    }

    #[test]
    fn test_sign_relative_paths() {
        let src = tempdir().unwrap();
        let out = tempdir().unwrap();
        let prefix = out.path().join("cs").to_string_lossy().into_owned();

        write_file(&src.path().join("file.bin"), b"data");

        run(SignArgs {
            folder: src.path(),
            prefix: &prefix,
        })
        .unwrap();

        let md5_content = fs::read_to_string(format!("{}.md5", prefix)).unwrap();
        let line = md5_content.lines().next().unwrap();
        let path_part = line.splitn(2, "  ").nth(1).unwrap();
        assert!(!path_part.starts_with('/'), "path must be relative, got: {}", path_part);
        assert!(path_part.contains("file.bin"));
    }

    #[test]
    fn test_sign_skips_md5_sha1_files() {
        let src = tempdir().unwrap();
        let out = tempdir().unwrap();
        let prefix = out.path().join("cs").to_string_lossy().into_owned();

        write_file(&src.path().join("real.bin"), b"data");
        write_file(&src.path().join("prev.md5"), b"old_hash");
        write_file(&src.path().join("prev.sha1"), b"old_hash");

        run(SignArgs {
            folder: src.path(),
            prefix: &prefix,
        })
        .unwrap();

        let md5_content = fs::read_to_string(format!("{}.md5", prefix)).unwrap();
        assert_eq!(md5_content.lines().count(), 1, "only real.bin should be hashed");
        assert!(!md5_content.contains("prev.md5"));
        assert!(!md5_content.contains("prev.sha1"));
    }
}
