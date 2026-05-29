use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use chrono::{DateTime, Local, NaiveDateTime};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct Commit {
    pub hash: String,
    pub email: String,
    pub author_name: String,
    pub subject: String,
    pub date: DateTime<Local>,
    pub repo: PathBuf,
}

pub type AuthorGroups = HashMap<String, HashMap<PathBuf, Vec<Commit>>>;

const SKIP_DIRS: &[&str] = &[".git", "node_modules", "target", ".cargo"];

pub fn scan_repos(dir: &Path) -> Vec<PathBuf> {
    let mut repos = Vec::new();
    let walker = WalkDir::new(dir)
        .follow_links(true)
        .into_iter()
        .filter_entry(|e| {
            !SKIP_DIRS.contains(&e.file_name().to_str().unwrap_or(""))
        });

    for entry in walker {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        if entry.file_type().is_dir() && entry.path().join(".git").exists() {
            repos.push(entry.path().to_path_buf());
        }
    }
    repos
}

pub fn git_log(
    repo: &Path,
    since: &DateTime<Local>,
    until: &DateTime<Local>,
) -> Result<Vec<Commit>, String> {
    let output = Command::new("git")
        .args([
            "-C",
            repo.to_str().ok_or_else(|| format!("non-UTF-8 repo path: {:?}", repo))?,
            "log",
            "--format=%H%x00%ae%x00%an%x00%s%x00%ad",
            "--date=format:%Y-%m-%dT%H:%M:%S",
            &format!("--after={}", (*since - chrono::Duration::seconds(1)).format("%Y-%m-%dT%H:%M:%S")),
            &format!("--before={}", (*until + chrono::Duration::seconds(1)).format("%Y-%m-%dT%H:%M:%S")),
        ])
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout
        .lines()
        .filter(|l| !l.is_empty())
        .filter_map(|line| parse_commit_line(line, repo))
        .collect())
}

fn parse_commit_line(line: &str, repo: &Path) -> Option<Commit> {
    let parts: Vec<&str> = line.splitn(5, '\0').collect();
    if parts.len() < 5 {
        return None;
    }
    let date = NaiveDateTime::parse_from_str(parts[4], "%Y-%m-%dT%H:%M:%S").ok()?;
    let date = date.and_local_timezone(Local)
        .earliest()
        .or_else(|| date.and_local_timezone(Local).latest())?;
    Some(Commit {
        hash: parts[0].to_string(),
        email: parts[1].to_string(),
        author_name: parts[2].to_string(),
        subject: parts[3].to_string(),
        date,
        repo: repo.to_path_buf(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_scan_repos_finds_git_dir() {
        let root = tempdir().unwrap();
        let repo = root.path().join("my-repo");
        fs::create_dir_all(repo.join(".git")).unwrap();

        let found = scan_repos(root.path());

        assert_eq!(found.len(), 1);
        assert_eq!(found[0], repo);
    }

    #[test]
    fn test_scan_repos_skips_node_modules() {
        let root = tempdir().unwrap();
        let nm_repo = root.path().join("node_modules").join("some-pkg");
        fs::create_dir_all(nm_repo.join(".git")).unwrap();

        let found = scan_repos(root.path());

        assert_eq!(found.len(), 0);
    }

    #[test]
    fn test_scan_repos_nested() {
        let root = tempdir().unwrap();
        fs::create_dir_all(root.path().join("a/.git")).unwrap();
        fs::create_dir_all(root.path().join("b/c/.git")).unwrap();

        let mut found = scan_repos(root.path());
        found.sort();

        assert_eq!(found.len(), 2);
    }

    #[test]
    fn test_parse_commit_line_valid() {
        let line = "abc1234\x00user@example.com\x00User Name\x00feat: add thing\x002026-05-28T10:30:00";
        let repo = PathBuf::from("/tmp/repo");
        let commit = parse_commit_line(line, &repo).unwrap();

        assert_eq!(commit.hash, "abc1234");
        assert_eq!(commit.email, "user@example.com");
        assert_eq!(commit.author_name, "User Name");
        assert_eq!(commit.subject, "feat: add thing");
        assert_eq!(
            commit.date.format("%Y-%m-%dT%H:%M:%S").to_string(),
            "2026-05-28T10:30:00"
        );
    }

    #[test]
    fn test_parse_commit_line_subject_with_pipe() {
        let line = "abc1234\x00user@example.com\x00User Name\x00fix: a|b edge case\x002026-05-28T10:30:00";
        let repo = PathBuf::from("/tmp/repo");
        let commit = parse_commit_line(line, &repo).unwrap();

        assert_eq!(commit.subject, "fix: a|b edge case");
    }

    #[test]
    fn test_parse_commit_line_invalid() {
        let line = "bad\x00line";
        let repo = PathBuf::from("/tmp/repo");
        assert!(parse_commit_line(line, &repo).is_none());
    }
}
