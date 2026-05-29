use std::fs;
use std::path::Path;

pub fn run(dir: &Path, even: bool) -> Result<(), String> {
    let mut entries: Vec<_> = fs::read_dir(dir)
        .map_err(|e| format!("error reading directory {}: {}", dir.display(), e))?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .collect();
    entries.sort();

    for (i, path) in entries.iter().enumerate() {
        let should_delete = if even { i % 2 == 0 } else { i % 2 != 0 };
        if should_delete {
            fs::remove_file(path)
                .map_err(|e| format!("error deleting {}: {}", path.display(), e))?;
            println!("Deleted: {}", path.display());
        } else {
            println!("Kept: {}", path.display());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    fn create_files(dir: &Path, names: &[&str]) {
        for name in names {
            File::create(dir.join(name)).unwrap();
        }
    }

    #[test]
    fn test_deleter_even_deletes_even_indices() {
        let dir = tempdir().unwrap();
        create_files(dir.path(), &["a.txt", "b.txt", "c.txt", "d.txt", "e.txt"]);

        run(dir.path(), true).unwrap();

        // sorted: a(0), b(1), c(2), d(3), e(4) — even 0,2,4 deleted
        assert!(!dir.path().join("a.txt").exists());
        assert!(dir.path().join("b.txt").exists());
        assert!(!dir.path().join("c.txt").exists());
        assert!(dir.path().join("d.txt").exists());
        assert!(!dir.path().join("e.txt").exists());
    }

    #[test]
    fn test_deleter_odd_deletes_odd_indices() {
        let dir = tempdir().unwrap();
        create_files(dir.path(), &["a.txt", "b.txt", "c.txt", "d.txt"]);

        run(dir.path(), false).unwrap();

        // sorted: a(0), b(1), c(2), d(3) — odd 1,3 deleted
        assert!(dir.path().join("a.txt").exists());
        assert!(!dir.path().join("b.txt").exists());
        assert!(dir.path().join("c.txt").exists());
        assert!(!dir.path().join("d.txt").exists());
    }

    #[test]
    fn test_deleter_missing_dir_returns_err() {
        let result = run(Path::new("/nonexistent/dir"), true);
        assert!(result.is_err());
    }
}
