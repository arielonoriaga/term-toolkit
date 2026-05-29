use sha1::{Digest, Sha1};
use std::fs;
use std::io::Write;
use std::path::Path;
use ttk_core::fs_utils::copy_clean_dir;
use walkdir::WalkDir;
use zip::write::SimpleFileOptions;

const SKIP_DIRS: &[&str] = &["node_modules", ".git", ".github", "dist"];
const SKIP_EXTS: &[&str] = &["md5", "sha1", "zip"];

pub struct BuildAndSignArgs<'a> {
    pub source: &'a Path,
    pub prefix: &'a str,
}

pub fn run(args: BuildAndSignArgs) -> Result<(), String> {
    if !args.source.exists() {
        return Err(format!("source not found: {}", args.source.display()));
    }

    let clean_dir = format!("{}-clean", args.prefix);
    let clean_path = Path::new(&clean_dir);

    if clean_path.exists() {
        return Err(format!("{} already exists", clean_dir));
    }

    copy_clean_dir(args.source, clean_path, SKIP_DIRS, SKIP_EXTS)?;

    let result = sign_and_zip(clean_path, args.prefix);

    if let Err(e) = fs::remove_dir_all(clean_path) {
        eprintln!("warn: could not remove temp dir {}: {}", clean_dir, e);
    }

    result
}

fn sign_and_zip(clean_path: &Path, prefix: &str) -> Result<(), String> {
    let mut md5_lines: Vec<String> = Vec::new();
    let mut sha1_lines: Vec<String> = Vec::new();

    for entry in WalkDir::new(clean_path).follow_links(false) {
        let entry = entry.map_err(|e| format!("walk error: {}", e))?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let bytes = fs::read(path).map_err(|e| format!("read {}: {}", path.display(), e))?;

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

    let md5_path = format!("{}.md5", prefix);
    let sha1_path = format!("{}.sha1", prefix);
    let zip_path = format!("{}.zip", prefix);

    fs::write(&md5_path, md5_lines.join("\n") + "\n")
        .map_err(|e| format!("write {}: {}", md5_path, e))?;
    fs::write(&sha1_path, sha1_lines.join("\n") + "\n")
        .map_err(|e| format!("write {}: {}", sha1_path, e))?;

    zip_dir(clean_path, &zip_path)?;

    println!("created {}, {}, {}", zip_path, md5_path, sha1_path);
    Ok(())
}

fn zip_dir(dir: &Path, zip_path: &str) -> Result<(), String> {
    let file = fs::File::create(zip_path)
        .map_err(|e| format!("create zip {}: {}", zip_path, e))?;
    let mut zip = zip::ZipWriter::new(file);
    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);

    for entry in WalkDir::new(dir).follow_links(false) {
        let entry = entry.map_err(|e| format!("walk error: {}", e))?;
        let path = entry.path();
        let rel = path
            .strip_prefix(dir)
            .map_err(|e| format!("strip prefix: {}", e))?;

        if path.is_dir() {
            if rel.as_os_str().is_empty() {
                continue;
            }
            let dir_name = rel.to_string_lossy().into_owned() + "/";
            zip.add_directory(&dir_name, options)
                .map_err(|e| format!("zip add dir {}: {}", dir_name, e))?;
        } else {
            let file_name = rel.to_string_lossy().into_owned();
            zip.start_file(&file_name, options)
                .map_err(|e| format!("zip start file {}: {}", file_name, e))?;
            let bytes = fs::read(path)
                .map_err(|e| format!("read {}: {}", path.display(), e))?;
            zip.write_all(&bytes)
                .map_err(|e| format!("zip write {}: {}", file_name, e))?;
        }
    }

    zip.finish().map_err(|e| format!("zip finish: {}", e))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write as IoWrite;
    use tempfile::tempdir;

    fn write_file(path: &Path, content: &str) {
        let mut f = File::create(path).unwrap();
        f.write_all(content.as_bytes()).unwrap();
    }

    #[test]
    fn test_build_and_sign_missing_source_err() {
        let tmp = tempdir().unwrap();
        let prefix = tmp.path().join("out").to_string_lossy().into_owned();
        let result = run(BuildAndSignArgs {
            source: &tmp.path().join("nope"),
            prefix: &prefix,
        });
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("source not found"));
    }

    #[test]
    fn test_build_and_sign_clean_dir_exists_err() {
        let src = tempdir().unwrap();
        let tmp = tempdir().unwrap();
        let prefix = tmp.path().join("out").to_string_lossy().into_owned();
        let clean_dir = format!("{}-clean", prefix);
        fs::create_dir_all(&clean_dir).unwrap();

        let result = run(BuildAndSignArgs {
            source: src.path(),
            prefix: &prefix,
        });

        fs::remove_dir_all(&clean_dir).ok();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("already exists"));
    }

    #[test]
    fn test_build_and_sign_produces_artifacts() {
        let src = tempdir().unwrap();
        let out = tempdir().unwrap();
        let prefix = out.path().join("release").to_string_lossy().into_owned();

        write_file(&src.path().join("main.rs"), "fn main() {}");
        write_file(&src.path().join("skip.md5"), "hash");

        run(BuildAndSignArgs {
            source: src.path(),
            prefix: &prefix,
        })
        .unwrap();

        assert!(std::path::Path::new(&format!("{}.zip", prefix)).exists());
        assert!(std::path::Path::new(&format!("{}.md5", prefix)).exists());
        assert!(std::path::Path::new(&format!("{}.sha1", prefix)).exists());
    }

    #[test]
    fn test_build_and_sign_temp_dir_cleaned() {
        let src = tempdir().unwrap();
        let out = tempdir().unwrap();
        let prefix = out.path().join("rel").to_string_lossy().into_owned();
        let clean_dir = format!("{}-clean", prefix);

        write_file(&src.path().join("f.txt"), "x");

        run(BuildAndSignArgs {
            source: src.path(),
            prefix: &prefix,
        })
        .unwrap();

        assert!(!std::path::Path::new(&clean_dir).exists(), "temp dir must be removed");
    }
}
