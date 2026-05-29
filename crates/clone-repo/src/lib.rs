use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct CloneArgs<'a> {
    pub url: &'a str,
    pub output: Option<&'a Path>,
    pub reset_history: bool,
}

pub fn run(args: CloneArgs) -> Result<(), String> {
    let repo_name = args.url
        .split('/')
        .last()
        .unwrap_or("repo")
        .trim_end_matches(".git");

    let target: PathBuf = args
        .output
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from(repo_name));

    let status = Command::new("git")
        .args(["clone", args.url, target.to_str().unwrap()])
        .status()
        .map_err(|e| e.to_string())?;
    if !status.success() {
        return Err("git clone failed".to_string());
    }

    if args.reset_history {
        fs::remove_dir_all(target.join(".git")).map_err(|e| e.to_string())?;

        for cmd_args in [
            vec!["init"],
            vec!["config", "user.email", "noreply@ttk"],
            vec!["config", "user.name", "ttk"],
            vec!["add", "."],
            vec!["commit", "-m", "Initial commit"],
        ] {
            let s = Command::new("git")
                .args(&cmd_args)
                .current_dir(&target)
                .status()
                .map_err(|e| e.to_string())?;
            if !s.success() {
                return Err(format!("git {} failed", cmd_args[0]));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn init_local_repo(path: &Path) {
        Command::new("git").args(["init", path.to_str().unwrap()]).output().unwrap();
        Command::new("git").args(["-C", path.to_str().unwrap(), "config", "user.email", "t@t.com"]).output().unwrap();
        Command::new("git").args(["-C", path.to_str().unwrap(), "config", "user.name", "T"]).output().unwrap();
        fs::write(path.join("file.txt"), "hello").unwrap();
        Command::new("git").args(["-C", path.to_str().unwrap(), "add", "."]).output().unwrap();
        Command::new("git").args(["-C", path.to_str().unwrap(), "commit", "-m", "init"]).output().unwrap();
    }

    #[test]
    fn test_clone_basic() {
        let source = tempdir().unwrap();
        let dest = tempdir().unwrap();
        let clone_dir = dest.path().join("cloned");
        init_local_repo(source.path());

        run(CloneArgs {
            url: source.path().to_str().unwrap(),
            output: Some(&clone_dir),
            reset_history: false,
        })
        .unwrap();

        assert!(clone_dir.join(".git").exists());
        assert!(clone_dir.join("file.txt").exists());
    }

    #[test]
    fn test_clone_with_reset_removes_history() {
        let source = tempdir().unwrap();
        let dest = tempdir().unwrap();
        let clone_dir = dest.path().join("cloned");
        init_local_repo(source.path());

        run(CloneArgs {
            url: source.path().to_str().unwrap(),
            output: Some(&clone_dir),
            reset_history: true,
        })
        .unwrap();

        // fresh git init → only 1 commit
        let log = Command::new("git")
            .args(["-C", clone_dir.to_str().unwrap(), "log", "--oneline"])
            .output()
            .unwrap();
        let stdout = String::from_utf8_lossy(&log.stdout);
        let lines: Vec<&str> = stdout.lines().collect();
        assert_eq!(lines.len(), 1);
        assert!(stdout.contains("Initial commit"));
    }
}
