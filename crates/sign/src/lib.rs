use sha1::{Digest, Sha1};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub struct SignArgs<'a> {
    pub folder: &'a Path,
    pub prefix: &'a str,
}

pub fn run(args: SignArgs) -> Result<(), String> {
    if !args.folder.exists() {
        return Err(format!("folder not found: {}", args.folder.display()));
    }

    let mut md5_lines: Vec<String> = Vec::new();
    let mut sha1_lines: Vec<String> = Vec::new();

    for entry in WalkDir::new(args.folder).follow_links(false) {
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

        let display = path.display().to_string();
        md5_lines.push(format!("{}  {}", md5_hash, display));
        sha1_lines.push(format!("{}  {}", sha1_hash, display));
    }

    let md5_path = format!("{}.md5", args.prefix);
    let sha1_path = format!("{}.sha1", args.prefix);

    fs::write(&md5_path, md5_lines.join("\n") + "\n")
        .map_err(|e| format!("write {}: {}", md5_path, e))?;
    fs::write(&sha1_path, sha1_lines.join("\n") + "\n")
        .map_err(|e| format!("write {}: {}", sha1_path, e))?;

    println!("wrote {} and {}", md5_path, sha1_path);
    Ok(())
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
}
