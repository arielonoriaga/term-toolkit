use std::path::Path;
use chrono::{DateTime, Local};
use crate::git::AuthorGroups;

pub struct MarkdownWriter;

impl MarkdownWriter {
    pub fn write(
        groups: &AuthorGroups,
        path: &Path,
        since: &DateTime<Local>,
        until: &DateTime<Local>,
        total_repos: usize,
        active_repos: usize,
    ) -> std::io::Result<()> {
        let mut md = format!(
            "# Git Digest — {}\n\n**Period:** {} → {}\n**Repos scanned:** {} | **With activity:** {}\n\n---\n",
            chrono::Local::now().format("%Y-%m-%d"),
            since.format("%Y-%m-%d"),
            until.format("%Y-%m-%d"),
            total_repos,
            active_repos,
        );

        let mut authors: Vec<&String> = groups.keys().collect();
        authors.sort();

        for email in authors {
            let repos = &groups[email];
            let name = repos
                .values()
                .flat_map(|c| c.iter())
                .next()
                .map(|c| c.author_name.as_str())
                .unwrap_or("unknown");
            md.push_str(&format!("\n## {}\n", name));

            let mut repo_paths: Vec<&std::path::PathBuf> = repos.keys().collect();
            repo_paths.sort();

            for repo in repo_paths {
                let commits = &repos[repo];
                md.push_str(&format!("\n### `{}`\n", repo.display()));
                md.push_str("| hash | message | date |\n|------|---------|------|\n");
                for c in commits {
                    md.push_str(&format!(
                        "| `{}` | {} | {} |\n",
                        &c.hash[..7.min(c.hash.len())],
                        c.subject,
                        c.date.format("%Y-%m-%d %H:%M"),
                    ));
                }
            }
        }

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, md)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::git::Commit;
    use chrono::TimeZone;
    use std::collections::HashMap;
    use std::path::PathBuf;
    use tempfile::tempdir;

    fn make_groups() -> AuthorGroups {
        let repo = PathBuf::from("/tmp/my-repo");
        let commit = Commit {
            hash: "abc1234def".to_string(),
            email: "user@test.com".to_string(),
            author_name: "Test User".to_string(),
            subject: "feat: add thing".to_string(),
            date: chrono::Local.with_ymd_and_hms(2026, 5, 28, 10, 30, 0).unwrap(),
            repo: repo.clone(),
        };
        let mut repo_map = HashMap::new();
        repo_map.insert(repo, vec![commit]);
        let mut groups = HashMap::new();
        groups.insert("user@test.com".to_string(), repo_map);
        groups
    }

    #[test]
    fn test_markdown_write_creates_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("digest.md");
        let groups = make_groups();
        let since = chrono::Local.with_ymd_and_hms(2026, 5, 27, 0, 0, 0).unwrap();
        let until = chrono::Local.with_ymd_and_hms(2026, 5, 28, 23, 59, 59).unwrap();

        MarkdownWriter::write(&groups, &path, &since, &until, 3, 1).unwrap();

        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.contains("# Git Digest"));
        assert!(content.contains("Test User"));
        assert!(content.contains("abc1234"));
        assert!(content.contains("feat: add thing"));
    }

    #[test]
    fn test_markdown_creates_parent_dirs() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("nested/dir/digest.md");
        let groups = make_groups();
        let since = chrono::Local.with_ymd_and_hms(2026, 5, 27, 0, 0, 0).unwrap();
        let until = chrono::Local.with_ymd_and_hms(2026, 5, 28, 23, 59, 59).unwrap();

        MarkdownWriter::write(&groups, &path, &since, &until, 1, 1).unwrap();

        assert!(path.exists());
    }

    #[test]
    fn test_markdown_output_sorted_by_author() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("digest.md");

        let mut groups = AuthorGroups::new();
        for (email, name) in [("z@z.com", "Z Author"), ("a@a.com", "A Author")] {
            let repo = PathBuf::from(format!("/tmp/{}", name));
            let commit = Commit {
                hash: "aaa1111".to_string(),
                email: email.to_string(),
                author_name: name.to_string(),
                subject: "test".to_string(),
                date: chrono::Local.with_ymd_and_hms(2026, 5, 28, 10, 0, 0).unwrap(),
                repo: repo.clone(),
            };
            let mut repo_map = HashMap::new();
            repo_map.insert(repo, vec![commit]);
            groups.insert(email.to_string(), repo_map);
        }

        let since = chrono::Local.with_ymd_and_hms(2026, 5, 27, 0, 0, 0).unwrap();
        let until = chrono::Local.with_ymd_and_hms(2026, 5, 28, 23, 59, 59).unwrap();
        MarkdownWriter::write(&groups, &path, &since, &until, 2, 2).unwrap();

        let content = std::fs::read_to_string(&path).unwrap();
        let a_pos = content.find("A Author").unwrap();
        let z_pos = content.find("Z Author").unwrap();
        assert!(a_pos < z_pos, "A Author should appear before Z Author");
    }
}
