use std::fs;
use std::path::Path;

pub fn run(dir: &Path, base_name: &str) -> Result<(), String> {
    let mut entries: Vec<_> = fs::read_dir(dir)
        .map_err(|e| format!("error reading directory {}: {}", dir.display(), e))?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .collect();
    entries.sort();

    let pad_len = entries.len().to_string().len();

    for (i, path) in entries.iter().enumerate() {
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| format!(".{}", e))
            .unwrap_or_default();
        let new_name = format!("{}{:0>width$}{}", base_name, i, ext, width = pad_len);
        let new_path = dir.join(&new_name);
        if new_path.exists() && new_path != *path {
            return Err(format!("collision: {} already exists", new_path.display()));
        }
        fs::rename(path, &new_path)
            .map_err(|e| format!("error renaming {}: {}", path.display(), e))?;
        println!("Renamed {} → {}", path.display(), new_path.display());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_rename_sequence_basic() {
        let dir = tempdir().unwrap();
        File::create(dir.path().join("c.txt")).unwrap();
        File::create(dir.path().join("a.txt")).unwrap();
        File::create(dir.path().join("b.txt")).unwrap();

        run(dir.path(), "file").unwrap();

        // sorted order: a(0), b(1), c(2)
        assert!(dir.path().join("file0.txt").exists());
        assert!(dir.path().join("file1.txt").exists());
        assert!(dir.path().join("file2.txt").exists());
    }

    #[test]
    fn test_rename_pads_index() {
        let dir = tempdir().unwrap();
        for i in 0..10 {
            File::create(dir.path().join(format!("f{}.txt", i))).unwrap();
        }

        run(dir.path(), "x").unwrap();

        assert!(dir.path().join("x00.txt").exists());
        assert!(dir.path().join("x09.txt").exists());
    }

    #[test]
    fn test_rename_preserves_extension() {
        let dir = tempdir().unwrap();
        File::create(dir.path().join("photo.jpg")).unwrap();
        File::create(dir.path().join("doc.pdf")).unwrap();

        run(dir.path(), "item").unwrap();

        let mut entries: Vec<_> = fs::read_dir(dir.path())
            .unwrap()
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_str().unwrap().to_string())
            .collect();
        entries.sort();

        assert!(entries.iter().any(|n| n.ends_with(".jpg")));
        assert!(entries.iter().any(|n| n.ends_with(".pdf")));
    }
}
