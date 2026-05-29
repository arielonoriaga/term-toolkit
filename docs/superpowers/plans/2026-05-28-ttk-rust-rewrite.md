# TTK Rust Rewrite + git-digest Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Rewrite term-toolkit as a Cargo workspace in Rust, porting all 4 existing commands and adding `git-digest`.

**Architecture:** Cargo workspace with `crates/core` (shared lib: git ops, output, markdown, prompts), `crates/cli` (binary entry with clap), and one crate per command. All commands compile into a single `ttk` binary.

**Tech Stack:** Rust, clap 4 (derive), inquire 0.7, walkdir 2, chrono 0.4, colored 2, indicatif 0.17, image 0.25, tempfile 3 (dev)

---

## File Map

| File | Purpose |
|---|---|
| `Cargo.toml` | workspace root |
| `crates/core/Cargo.toml` | core lib deps |
| `crates/core/src/lib.rs` | pub mod re-exports |
| `crates/core/src/git.rs` | `Commit`, `AuthorGroups`, `scan_repos`, `git_log` |
| `crates/core/src/output.rs` | `TermOutput::print_digest` |
| `crates/core/src/markdown.rs` | `MarkdownWriter::write` |
| `crates/core/src/prompt.rs` | `AuthorSummary`, `Prompt::date_range`, `Prompt::select_authors` |
| `crates/cli/Cargo.toml` | cli bin deps |
| `crates/cli/src/main.rs` | clap `Cli` + `Commands`, dispatch |
| `crates/deleter/Cargo.toml` | |
| `crates/deleter/src/lib.rs` | `run(dir, even)` |
| `crates/rename/Cargo.toml` | |
| `crates/rename/src/lib.rs` | `run(dir, base_name)` |
| `crates/optimize/Cargo.toml` | image dep |
| `crates/optimize/src/lib.rs` | `run(OptimizeArgs)` |
| `crates/clone-repo/Cargo.toml` | |
| `crates/clone-repo/src/lib.rs` | `run(CloneArgs)` |
| `crates/git-digest/Cargo.toml` | core + indicatif deps |
| `crates/git-digest/src/lib.rs` | `run(GitDigestArgs)`, `resolve_last` |

---

## Task 1: Workspace scaffold

**Files:**
- Create: `Cargo.toml`
- Create: `crates/core/Cargo.toml`
- Create: `crates/core/src/lib.rs`
- Create: `crates/cli/Cargo.toml`
- Create: `crates/cli/src/main.rs`
- Create: `crates/deleter/Cargo.toml`, `crates/deleter/src/lib.rs`
- Create: `crates/rename/Cargo.toml`, `crates/rename/src/lib.rs`
- Create: `crates/optimize/Cargo.toml`, `crates/optimize/src/lib.rs`
- Create: `crates/clone-repo/Cargo.toml`, `crates/clone-repo/src/lib.rs`
- Create: `crates/git-digest/Cargo.toml`, `crates/git-digest/src/lib.rs`
- Modify: `.gitignore`

- [ ] **Step 1: Create workspace `Cargo.toml`**

```toml
[workspace]
members = [
    "crates/cli",
    "crates/core",
    "crates/deleter",
    "crates/rename",
    "crates/optimize",
    "crates/clone-repo",
    "crates/git-digest",
]
resolver = "2"
```

- [ ] **Step 2: Create `crates/core/Cargo.toml`**

```toml
[package]
name = "ttk-core"
version = "0.1.0"
edition = "2021"

[dependencies]
walkdir = "2"
chrono = "0.4"
colored = "2"
inquire = "0.7"

[dev-dependencies]
tempfile = "3"
```

- [ ] **Step 3: Create `crates/core/src/lib.rs`** (placeholder, expanded in Task 2)

```rust
pub mod git;
pub mod markdown;
pub mod output;
pub mod prompt;
```

- [ ] **Step 4: Create `crates/cli/Cargo.toml`**

```toml
[package]
name = "ttk"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "ttk"
path = "src/main.rs"

[dependencies]
clap = { version = "4", features = ["derive"] }
chrono = "0.4"
ttk-core = { path = "../core" }
ttk-deleter = { path = "../deleter" }
ttk-rename = { path = "../rename" }
ttk-optimize = { path = "../optimize" }
ttk-clone-repo = { path = "../clone-repo" }
ttk-git-digest = { path = "../git-digest" }
```

- [ ] **Step 5: Create `crates/cli/src/main.rs`** (stub, wired in Task 10)

```rust
fn main() {
    println!("ttk");
}
```

- [ ] **Step 6: Create remaining crate stubs**

`crates/deleter/Cargo.toml`:
```toml
[package]
name = "ttk-deleter"
version = "0.1.0"
edition = "2021"

[dev-dependencies]
tempfile = "3"
```

`crates/deleter/src/lib.rs`:
```rust
pub fn run(_dir: &std::path::Path, _even: bool) -> Result<(), String> { Ok(()) }
```

`crates/rename/Cargo.toml`:
```toml
[package]
name = "ttk-rename"
version = "0.1.0"
edition = "2021"

[dev-dependencies]
tempfile = "3"
```

`crates/rename/src/lib.rs`:
```rust
pub fn run(_dir: &std::path::Path, _base_name: &str) -> Result<(), String> { Ok(()) }
```

`crates/optimize/Cargo.toml`:
```toml
[package]
name = "ttk-optimize"
version = "0.1.0"
edition = "2021"

[dependencies]
image = "0.25"

[dev-dependencies]
tempfile = "3"
```

`crates/optimize/src/lib.rs`:
```rust
pub struct OptimizeArgs<'a> {
    pub input: &'a std::path::Path,
    pub output: Option<&'a std::path::Path>,
    pub quality: u8,
    pub keep_original: bool,
}
pub fn run(_args: OptimizeArgs) -> Result<(), String> { Ok(()) }
```

`crates/clone-repo/Cargo.toml`:
```toml
[package]
name = "ttk-clone-repo"
version = "0.1.0"
edition = "2021"
```

`crates/clone-repo/src/lib.rs`:
```rust
pub struct CloneArgs<'a> {
    pub url: &'a str,
    pub output: Option<&'a std::path::Path>,
    pub reset_history: bool,
}
pub fn run(_args: CloneArgs) -> Result<(), String> { Ok(()) }
```

`crates/git-digest/Cargo.toml`:
```toml
[package]
name = "ttk-git-digest"
version = "0.1.0"
edition = "2021"

[dependencies]
ttk-core = { path = "../core" }
chrono = "0.4"
indicatif = "0.17"
colored = "2"
```

`crates/git-digest/src/lib.rs`:
```rust
pub fn run() {}
```

Also create placeholder `src/` modules so core compiles:

`crates/core/src/git.rs`:
```rust
```

`crates/core/src/output.rs`:
```rust
```

`crates/core/src/markdown.rs`:
```rust
```

`crates/core/src/prompt.rs`:
```rust
```

- [ ] **Step 7: Add `target/` to `.gitignore`**

Append to existing `.gitignore`:
```
/target
```

- [ ] **Step 8: Verify workspace compiles**

```bash
cd /home/ariel/term-toolkit && cargo check
```

Expected: all 7 crates compile with 0 errors.

- [ ] **Step 9: Commit**

```bash
git add Cargo.toml Cargo.lock crates/ .gitignore
git commit -m "feat: scaffold cargo workspace with 7 crates"
```

---

## Task 2: ttk-core — git module

**Files:**
- Create: `crates/core/src/git.rs`
- Test: inline `#[cfg(test)]` block

- [ ] **Step 1: Write failing tests**

Replace `crates/core/src/git.rs` with:

```rust
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
        let line = "abc1234|user@example.com|User Name|feat: add thing|2026-05-28T10:30:00";
        let repo = PathBuf::from("/tmp/repo");
        let commit = parse_commit_line(line, &repo).unwrap();

        assert_eq!(commit.hash, "abc1234");
        assert_eq!(commit.email, "user@example.com");
        assert_eq!(commit.author_name, "User Name");
        assert_eq!(commit.subject, "feat: add thing");
    }

    #[test]
    fn test_parse_commit_line_subject_with_pipe() {
        // subject containing | should still parse (splitn(5, ..))
        let line = "abc1234|user@example.com|User Name|fix: a|b edge case|2026-05-28T10:30:00";
        let repo = PathBuf::from("/tmp/repo");
        let commit = parse_commit_line(line, &repo).unwrap();

        assert_eq!(commit.subject, "fix: a|b edge case");
    }

    #[test]
    fn test_parse_commit_line_invalid() {
        let line = "bad|line";
        let repo = PathBuf::from("/tmp/repo");
        assert!(parse_commit_line(line, &repo).is_none());
    }
}
```

- [ ] **Step 2: Run tests to verify they fail**

```bash
cd /home/ariel/term-toolkit && cargo test -p ttk-core 2>&1 | head -20
```

Expected: compile error — `scan_repos`, `parse_commit_line` not defined.

- [ ] **Step 3: Implement `scan_repos` and `git_log`**

Replace `crates/core/src/git.rs` content (keep tests block at bottom, add before it):

```rust
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
            repo.to_str().unwrap_or(""),
            "log",
            "--format=%H|%ae|%an|%s|%ad",
            "--date=format:%Y-%m-%dT%H:%M:%S",
            &format!("--after={}", since.format("%Y-%m-%dT%H:%M:%S")),
            &format!("--before={}", until.format("%Y-%m-%dT%H:%M:%S")),
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
    let parts: Vec<&str> = line.splitn(5, '|').collect();
    if parts.len() < 5 {
        return None;
    }
    let date = NaiveDateTime::parse_from_str(parts[4], "%Y-%m-%dT%H:%M:%S").ok()?;
    let date = date.and_local_timezone(Local).single()?;
    Some(Commit {
        hash: parts[0].to_string(),
        email: parts[1].to_string(),
        author_name: parts[2].to_string(),
        subject: parts[3].to_string(),
        date,
        repo: repo.to_path_buf(),
    })
}
```

- [ ] **Step 4: Run tests to verify they pass**

```bash
cd /home/ariel/term-toolkit && cargo test -p ttk-core git 2>&1
```

Expected: `test_scan_repos_finds_git_dir ... ok`, `test_parse_commit_line_valid ... ok`, etc.

- [ ] **Step 5: Commit**

```bash
git add crates/core/src/git.rs
git commit -m "feat(core): add scan_repos, git_log, Commit"
```

---

## Task 3: ttk-core — output + markdown modules

**Files:**
- Create: `crates/core/src/output.rs`
- Create: `crates/core/src/markdown.rs`

- [ ] **Step 1: Write failing test for MarkdownWriter**

Replace `crates/core/src/markdown.rs`:

```rust
use std::collections::HashMap;
use std::path::Path;
use chrono::{DateTime, Local, TimeZone};
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
        todo!()
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
            date: Local.with_ymd_and_hms(2026, 5, 28, 10, 30, 0).unwrap(),
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
        let since = Local.with_ymd_and_hms(2026, 5, 27, 0, 0, 0).unwrap();
        let until = Local.with_ymd_and_hms(2026, 5, 28, 23, 59, 59).unwrap();

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
        let since = Local.with_ymd_and_hms(2026, 5, 27, 0, 0, 0).unwrap();
        let until = Local.with_ymd_and_hms(2026, 5, 28, 23, 59, 59).unwrap();

        MarkdownWriter::write(&groups, &path, &since, &until, 1, 1).unwrap();

        assert!(path.exists());
    }
}
```

- [ ] **Step 2: Run test to verify it fails**

```bash
cd /home/ariel/term-toolkit && cargo test -p ttk-core markdown 2>&1 | tail -5
```

Expected: panics with `not yet implemented` (todo!).

- [ ] **Step 3: Implement `MarkdownWriter::write`**

Replace `todo!()` in `MarkdownWriter::write` with:

```rust
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
            Local::now().format("%Y-%m-%d"),
            since.format("%Y-%m-%d"),
            until.format("%Y-%m-%d"),
            total_repos,
            active_repos,
        );

        for (_, repos) in groups {
            let name = repos
                .values()
                .flat_map(|c| c.iter())
                .next()
                .map(|c| c.author_name.as_str())
                .unwrap_or("unknown");
            md.push_str(&format!("\n## {}\n", name));

            for (repo, commits) in repos {
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
```

- [ ] **Step 4: Implement `TermOutput`**

Replace `crates/core/src/output.rs`:

```rust
use colored::Colorize;
use crate::git::AuthorGroups;

pub struct TermOutput;

impl TermOutput {
    pub fn print_digest(groups: &AuthorGroups) {
        for (_, repos) in groups {
            let name = repos
                .values()
                .flat_map(|c| c.iter())
                .next()
                .map(|c| c.author_name.as_str())
                .unwrap_or("unknown");

            println!("\n{}", format!("## {}", name).bold().green());

            for (repo, commits) in repos {
                println!("  {}", repo.display().to_string().cyan());
                for c in commits {
                    println!(
                        "    {} {}",
                        c.hash[..7.min(c.hash.len())].yellow(),
                        c.subject
                    );
                }
            }
        }
    }
}
```

- [ ] **Step 5: Run tests**

```bash
cd /home/ariel/term-toolkit && cargo test -p ttk-core 2>&1
```

Expected: all tests pass.

- [ ] **Step 6: Commit**

```bash
git add crates/core/src/output.rs crates/core/src/markdown.rs
git commit -m "feat(core): add TermOutput and MarkdownWriter"
```

---

## Task 4: ttk-core — prompt module

**Files:**
- Create: `crates/core/src/prompt.rs`

Prompt code uses `inquire` which requires a TTY — no unit tests. Tested manually in Task 11.

- [ ] **Step 1: Implement `crates/core/src/prompt.rs`**

```rust
use chrono::{DateTime, Duration, Local, NaiveDate, TimeZone};
use inquire::{MultiSelect, Select, Text};

pub struct AuthorSummary {
    pub email: String,
    pub name: String,
    pub commit_count: usize,
    pub repo_count: usize,
}

pub struct Prompt;

impl Prompt {
    pub fn date_range() -> (DateTime<Local>, DateTime<Local>) {
        let options = vec![
            "Last 24 hours",
            "Last 7 days",
            "Last 30 days",
            "Custom range",
        ];
        let choice = Select::new("Date range:", options)
            .prompt()
            .expect("prompt error");
        let now = Local::now();
        match choice {
            "Last 24 hours" => (now - Duration::hours(24), now),
            "Last 7 days" => (now - Duration::days(7), now),
            "Last 30 days" => (now - Duration::days(30), now),
            _ => {
                let since_str = Text::new("Since (YYYY-MM-DD):")
                    .prompt()
                    .expect("prompt error");
                let until_str = Text::new("Until (YYYY-MM-DD):")
                    .with_default(&now.format("%Y-%m-%d").to_string())
                    .prompt()
                    .expect("prompt error");
                let since = NaiveDate::parse_from_str(&since_str, "%Y-%m-%d")
                    .expect("invalid date format, use YYYY-MM-DD")
                    .and_hms_opt(0, 0, 0)
                    .unwrap()
                    .and_local_timezone(Local)
                    .single()
                    .expect("ambiguous local time");
                let until = NaiveDate::parse_from_str(&until_str, "%Y-%m-%d")
                    .expect("invalid date format, use YYYY-MM-DD")
                    .and_hms_opt(23, 59, 59)
                    .unwrap()
                    .and_local_timezone(Local)
                    .single()
                    .expect("ambiguous local time");
                (since, until)
            }
        }
    }

    pub fn select_authors(summaries: &[AuthorSummary]) -> Vec<String> {
        let options: Vec<String> = summaries
            .iter()
            .map(|a| format!("{} ({} commits, {} repos)", a.name, a.commit_count, a.repo_count))
            .collect();
        loop {
            let selected = MultiSelect::new("Select authors:", options.clone())
                .prompt()
                .expect("prompt error");
            if !selected.is_empty() {
                return summaries
                    .iter()
                    .zip(options.iter())
                    .filter(|(_, opt)| selected.contains(opt))
                    .map(|(a, _)| a.email.clone())
                    .collect();
            }
            eprintln!("Select at least one author.");
        }
    }
}
```

- [ ] **Step 2: Verify core compiles**

```bash
cd /home/ariel/term-toolkit && cargo check -p ttk-core 2>&1
```

Expected: 0 errors.

- [ ] **Step 3: Commit**

```bash
git add crates/core/src/prompt.rs
git commit -m "feat(core): add Prompt with date_range and select_authors"
```

---

## Task 5: ttk-deleter

**Files:**
- Modify: `crates/deleter/src/lib.rs`

- [ ] **Step 1: Write failing tests**

Replace `crates/deleter/src/lib.rs`:

```rust
use std::fs;
use std::path::Path;

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
```

- [ ] **Step 2: Run tests to verify they fail**

```bash
cd /home/ariel/term-toolkit && cargo test -p ttk-deleter 2>&1 | tail -5
```

Expected: compile error — `run` not defined.

- [ ] **Step 3: Implement `run`**

Add before the `#[cfg(test)]` block:

```rust
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
```

- [ ] **Step 4: Run tests**

```bash
cd /home/ariel/term-toolkit && cargo test -p ttk-deleter 2>&1
```

Expected: 3 tests pass.

- [ ] **Step 5: Commit**

```bash
git add crates/deleter/src/lib.rs
git commit -m "feat(deleter): port delete-by-index from TS"
```

---

## Task 6: ttk-rename

**Files:**
- Modify: `crates/rename/src/lib.rs`

- [ ] **Step 1: Write failing tests**

Replace `crates/rename/src/lib.rs`:

```rust
use std::fs;
use std::path::Path;

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
```

- [ ] **Step 2: Run tests to verify they fail**

```bash
cd /home/ariel/term-toolkit && cargo test -p ttk-rename 2>&1 | tail -5
```

Expected: compile error — `run` not defined.

- [ ] **Step 3: Implement `run`**

Add before `#[cfg(test)]`:

```rust
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
        fs::rename(path, &new_path)
            .map_err(|e| format!("error renaming {}: {}", path.display(), e))?;
        println!("Renamed {} → {}", path.display(), new_path.display());
    }
    Ok(())
}
```

- [ ] **Step 4: Run tests**

```bash
cd /home/ariel/term-toolkit && cargo test -p ttk-rename 2>&1
```

Expected: 3 tests pass.

- [ ] **Step 5: Commit**

```bash
git add crates/rename/src/lib.rs
git commit -m "feat(rename): port rename-sequence from TS"
```

---

## Task 7: ttk-optimize

**Files:**
- Modify: `crates/optimize/src/lib.rs`

Supports JPEG (lossy quality), PNG (lossless, compression level), and any other format `image` crate supports (re-encoded, quality ignored).

- [ ] **Step 1: Write failing tests**

Replace `crates/optimize/src/lib.rs`:

```rust
use image::{ImageFormat, ImageReader};
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};

pub struct OptimizeArgs<'a> {
    pub input: &'a Path,
    pub output: Option<&'a Path>,
    pub quality: u8,
    pub keep_original: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn tiny_jpeg(path: &Path) {
        let img = image::RgbImage::new(2, 2);
        img.save_with_format(path, image::ImageFormat::Jpeg).unwrap();
    }

    fn tiny_png(path: &Path) {
        let img = image::RgbImage::new(2, 2);
        img.save_with_format(path, image::ImageFormat::Png).unwrap();
    }

    #[test]
    fn test_optimize_jpeg_produces_valid_image() {
        let dir = tempdir().unwrap();
        let input = dir.path().join("test.jpg");
        let output = dir.path().join("out.jpg");
        tiny_jpeg(&input);

        run(OptimizeArgs {
            input: &input,
            output: Some(&output),
            quality: 50,
            keep_original: true,
        })
        .unwrap();

        assert!(output.exists());
        ImageReader::open(&output).unwrap().decode().unwrap();
    }

    #[test]
    fn test_optimize_png_produces_valid_image() {
        let dir = tempdir().unwrap();
        let input = dir.path().join("test.png");
        let output = dir.path().join("out.png");
        tiny_png(&input);

        run(OptimizeArgs {
            input: &input,
            output: Some(&output),
            quality: 80,
            keep_original: true,
        })
        .unwrap();

        assert!(output.exists());
        ImageReader::open(&output).unwrap().decode().unwrap();
    }

    #[test]
    fn test_optimize_overwrites_original_when_no_keep() {
        let dir = tempdir().unwrap();
        let input = dir.path().join("test.jpg");
        tiny_jpeg(&input);
        let original_size = fs::metadata(&input).unwrap().len();

        run(OptimizeArgs {
            input: &input,
            output: None,
            quality: 10,
            keep_original: false,
        })
        .unwrap();

        assert!(input.exists());
    }

    #[test]
    fn test_optimize_dir_processes_images() {
        let dir = tempdir().unwrap();
        tiny_jpeg(&dir.path().join("a.jpg"));
        tiny_png(&dir.path().join("b.png"));
        let out_dir = dir.path().join("out");
        fs::create_dir(&out_dir).unwrap();

        run(OptimizeArgs {
            input: dir.path(),
            output: Some(&out_dir),
            quality: 80,
            keep_original: true,
        })
        .unwrap();

        assert!(out_dir.join("a.jpg").exists());
        assert!(out_dir.join("b.png").exists());
    }
}
```

- [ ] **Step 2: Run tests to verify they fail**

```bash
cd /home/ariel/term-toolkit && cargo test -p ttk-optimize 2>&1 | tail -5
```

Expected: compile error — `run` not defined.

- [ ] **Step 3: Implement `run`**

Add before `#[cfg(test)]`:

```rust
use image::{ImageFormat, ImageReader};
use std::fs;
use std::io::{BufWriter, Cursor};
use std::path::{Path, PathBuf};

pub struct OptimizeArgs<'a> {
    pub input: &'a Path,
    pub output: Option<&'a Path>,
    pub quality: u8,
    pub keep_original: bool,
}

pub fn run(args: OptimizeArgs) -> Result<(), String> {
    let meta = fs::metadata(args.input).map_err(|e| e.to_string())?;
    if meta.is_file() {
        optimize_file(args.input, args.output, args.quality, args.keep_original)
    } else if meta.is_dir() {
        optimize_dir(args.input, args.output, args.quality, args.keep_original)
    } else {
        Err("path is neither a file nor a directory".to_string())
    }
}

fn optimize_file(
    input: &Path,
    output: Option<&Path>,
    quality: u8,
    keep_original: bool,
) -> Result<(), String> {
    let fmt = ImageFormat::from_path(input).map_err(|e| e.to_string())?;
    let img = ImageReader::open(input)
        .map_err(|e| e.to_string())?
        .decode()
        .map_err(|e| e.to_string())?;

    let dest = output.map(|p| p.to_path_buf()).unwrap_or_else(|| input.to_path_buf());

    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    match fmt {
        ImageFormat::Jpeg => {
            let mut buf = Vec::new();
            let mut enc = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buf, quality);
            enc.encode_image(&img).map_err(|e| e.to_string())?;
            let write_path = if output.is_none() && keep_original {
                input.to_path_buf()
            } else {
                dest
            };
            fs::write(&write_path, buf).map_err(|e| e.to_string())?;
        }
        _ => {
            img.save_with_format(&dest, fmt).map_err(|e| e.to_string())?;
            if output.is_none() && !keep_original {
                if dest != input {
                    fs::remove_file(input).map_err(|e| e.to_string())?;
                }
            }
        }
    }

    println!("Optimized: {}", dest.display());
    Ok(())
}

fn optimize_dir(
    input: &Path,
    output: Option<&Path>,
    quality: u8,
    keep_original: bool,
) -> Result<(), String> {
    let entries = fs::read_dir(input).map_err(|e| e.to_string())?;
    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
        let out_path = output.map(|o| o.join(entry.file_name()));
        let meta = fs::metadata(&path).map_err(|e| e.to_string())?;
        if meta.is_file() && ImageFormat::from_path(&path).is_ok() {
            optimize_file(&path, out_path.as_deref(), quality, keep_original)?;
        } else if meta.is_dir() {
            optimize_dir(&path, out_path.as_deref(), quality, keep_original)?;
        }
    }
    Ok(())
}
```

- [ ] **Step 4: Run tests**

```bash
cd /home/ariel/term-toolkit && cargo test -p ttk-optimize 2>&1
```

Expected: 4 tests pass.

- [ ] **Step 5: Commit**

```bash
git add crates/optimize/src/lib.rs
git commit -m "feat(optimize): port image optimizer from TS"
```

---

## Task 8: ttk-clone-repo

**Files:**
- Modify: `crates/clone-repo/src/lib.rs`

- [ ] **Step 1: Write failing test**

Replace `crates/clone-repo/src/lib.rs`:

```rust
use std::path::{Path, PathBuf};
use std::process::Command;
use std::fs;

pub struct CloneArgs<'a> {
    pub url: &'a str,
    pub output: Option<&'a Path>,
    pub reset_history: bool,
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
        let lines: Vec<_> = String::from_utf8_lossy(&log.stdout)
            .lines()
            .collect();
        assert_eq!(lines.len(), 1);
        assert!(String::from_utf8_lossy(&log.stdout).contains("Initial commit"));
    }
}
```

- [ ] **Step 2: Run tests to verify they fail**

```bash
cd /home/ariel/term-toolkit && cargo test -p ttk-clone-repo 2>&1 | tail -5
```

Expected: compile error — `run` not defined.

- [ ] **Step 3: Implement `run`**

Add before `#[cfg(test)]`:

```rust
use std::path::{Path, PathBuf};
use std::process::Command;
use std::fs;

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
```

- [ ] **Step 4: Run tests**

```bash
cd /home/ariel/term-toolkit && cargo test -p ttk-clone-repo 2>&1
```

Expected: 2 tests pass.

- [ ] **Step 5: Commit**

```bash
git add crates/clone-repo/src/lib.rs
git commit -m "feat(clone-repo): port git clone+reset from TS"
```

---

## Task 9: ttk-git-digest

**Files:**
- Modify: `crates/git-digest/src/lib.rs`

- [ ] **Step 1: Write failing test for `resolve_last`**

Replace `crates/git-digest/src/lib.rs`:

```rust
use chrono::{DateTime, Duration, Local, NaiveDate, TimeZone};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use ttk_core::git::{git_log, scan_repos, AuthorGroups, Commit};
use ttk_core::markdown::MarkdownWriter;
use ttk_core::output::TermOutput;
use ttk_core::prompt::{AuthorSummary, Prompt};

pub struct GitDigestArgs {
    pub dir: PathBuf,
    pub since: Option<DateTime<Local>>,
    pub until: Option<DateTime<Local>>,
    pub last: Option<String>,
    pub output: Option<PathBuf>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_last_24h() {
        let (since, until) = resolve_last("24h").unwrap();
        let diff = until - since;
        assert!(diff.num_hours() >= 23 && diff.num_hours() <= 25);
    }

    #[test]
    fn test_resolve_last_7d() {
        let (since, until) = resolve_last("7d").unwrap();
        let diff = until - since;
        assert_eq!(diff.num_days(), 7);
    }

    #[test]
    fn test_resolve_last_invalid() {
        assert!(resolve_last("3x").is_err());
        assert!(resolve_last("").is_err());
    }
}
```

- [ ] **Step 2: Run tests to verify they fail**

```bash
cd /home/ariel/term-toolkit && cargo test -p ttk-git-digest 2>&1 | tail -5
```

Expected: compile error — `resolve_last` not defined.

- [ ] **Step 3: Implement `resolve_last` and `run`**

Add before `#[cfg(test)]`:

```rust
pub fn resolve_last(last: &str) -> Result<(DateTime<Local>, DateTime<Local>), String> {
    let now = Local::now();
    let since = match last {
        "24h" => now - Duration::hours(24),
        "7d" => now - Duration::days(7),
        "30d" => now - Duration::days(30),
        other => return Err(format!("invalid duration '{}', use 24h|7d|30d", other)),
    };
    Ok((since, now))
}

pub fn run(args: GitDigestArgs) -> Result<(), String> {
    // Step 1: Scan
    if !args.dir.exists() {
        return Err(format!("directory not found: {}", args.dir.display()));
    }
    println!("{}", "Scanning for repositories...".dimmed());
    let repos = scan_repos(&args.dir);
    if repos.is_empty() {
        println!("No git repositories found in {}", args.dir.display());
        return Ok(());
    }
    println!("Found {} repositories", repos.len());

    // Resolve date range
    let (since, until) = if let Some(ref last) = args.last {
        resolve_last(last)?
    } else if args.since.is_some() || args.until.is_some() {
        let now = Local::now();
        (
            args.since.unwrap_or_else(|| now - Duration::days(30)),
            args.until.unwrap_or(now),
        )
    } else {
        Prompt::date_range()
    };

    if since > until {
        return Err("--since must be before --until".to_string());
    }

    // Step 2: Log with progress
    let pb = if repos.len() > 100 {
        let bar = ProgressBar::new(repos.len() as u64);
        bar.set_style(
            ProgressStyle::default_bar()
                .template("[{bar:40}] {pos}/{len} repos")
                .unwrap(),
        );
        Some(bar)
    } else {
        None
    };

    let mut all_commits: Vec<Commit> = Vec::new();
    for repo in &repos {
        match git_log(repo, &since, &until) {
            Ok(commits) => all_commits.extend(commits),
            Err(e) => eprintln!("⚠ skipped {} (git error: {})", repo.display(), e),
        }
        if let Some(ref bar) = pb { bar.inc(1); }
    }
    if let Some(bar) = pb { bar.finish_and_clear(); }

    if all_commits.is_empty() {
        println!("No commits found in the selected period.");
        return Ok(());
    }

    // Step 3: Build author summaries
    let mut by_author: HashMap<String, Vec<&Commit>> = HashMap::new();
    for commit in &all_commits {
        by_author.entry(commit.email.clone()).or_default().push(commit);
    }
    let summaries: Vec<AuthorSummary> = by_author
        .iter()
        .map(|(email, commits)| {
            let name = commits[0].author_name.clone();
            let commit_count = commits.len();
            let repo_count = commits
                .iter()
                .map(|c| &c.repo)
                .collect::<HashSet<_>>()
                .len();
            AuthorSummary { email: email.clone(), name, commit_count, repo_count }
        })
        .collect();

    // Step 4: Author selection
    let selected_emails = Prompt::select_authors(&summaries);

    // Step 5: Group by author → repo → commits
    let mut groups: AuthorGroups = HashMap::new();
    for commit in all_commits {
        if selected_emails.contains(&commit.email) {
            groups
                .entry(commit.email.clone())
                .or_default()
                .entry(commit.repo.clone())
                .or_default()
                .push(commit);
        }
    }

    if groups.is_empty() {
        println!("No commits found for selected authors.");
        return Ok(());
    }

    let active_repos = groups
        .values()
        .flat_map(|r| r.keys())
        .collect::<HashSet<_>>()
        .len();

    // Terminal output
    TermOutput::print_digest(&groups);

    // Markdown output
    let md_filename = format!("git-digest-{}.md", Local::now().format("%Y-%m-%d"));
    let md_dir = args.output.as_deref().unwrap_or(&args.dir);
    let md_path = md_dir.join(&md_filename);

    if md_path.exists() {
        eprintln!("⚠ overwriting {}", md_filename);
    }

    MarkdownWriter::write(&groups, &md_path, &since, &until, repos.len(), active_repos)
        .map_err(|e| e.to_string())?;

    println!("\nSaved to {}", md_path.display());
    Ok(())
}
```

- [ ] **Step 4: Run tests**

```bash
cd /home/ariel/term-toolkit && cargo test -p ttk-git-digest 2>&1
```

Expected: `test_resolve_last_24h ... ok`, `test_resolve_last_7d ... ok`, `test_resolve_last_invalid ... ok`.

- [ ] **Step 5: Commit**

```bash
git add crates/git-digest/src/lib.rs
git commit -m "feat(git-digest): implement scan → log → author select → render"
```

---

## Task 10: ttk-cli — wire all subcommands

**Files:**
- Modify: `crates/cli/src/main.rs`

- [ ] **Step 1: Replace `main.rs` with full clap routing**

```rust
use chrono::{DateTime, Local, NaiveDate, TimeZone};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "ttk", version, about = "Terminal Tool Kit")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Delete files by index (even or odd)
    Deleter {
        directory: PathBuf,
        /// Delete even-indexed files (default: odd)
        #[arg(short, long)]
        even: bool,
    },
    /// Rename files sequentially (file0.txt, file1.txt, ...)
    Rename {
        new_name: String,
        #[arg(short, long, default_value = ".")]
        directory: PathBuf,
    },
    /// Optimize images in a directory
    Optimize {
        directory: PathBuf,
        #[arg(short, long, default_value = "80")]
        quality: u8,
        #[arg(short, long)]
        output: Option<PathBuf>,
        #[arg(long, default_value_t = true)]
        keep_original: bool,
    },
    /// Clone a repo and optionally reset history
    CloneRepo {
        repo_url: String,
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Reset git history after clone (default: true)
        #[arg(long, default_value_t = true)]
        reset: bool,
    },
    /// Summarize commits across repos in a date range
    GitDigest {
        dir: PathBuf,
        /// Date string ISO 8601, e.g. 2026-05-01
        #[arg(long)]
        since: Option<String>,
        /// Date string ISO 8601, e.g. 2026-05-28
        #[arg(long)]
        until: Option<String>,
        /// Shorthand: 24h | 7d | 30d
        #[arg(long)]
        last: Option<String>,
        /// Output directory for markdown file
        #[arg(long)]
        output: Option<PathBuf>,
    },
}

fn parse_date(s: &str) -> DateTime<Local> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .unwrap_or_else(|_| panic!("invalid date '{}', use YYYY-MM-DD", s))
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_local_timezone(Local)
        .single()
        .expect("ambiguous local time")
}

fn main() {
    let cli = Cli::parse();
    let result = match cli.command {
        Commands::Deleter { directory, even } => ttk_deleter::run(&directory, even),
        Commands::Rename { new_name, directory } => ttk_rename::run(&directory, &new_name),
        Commands::Optimize { directory, quality, output, keep_original } => {
            ttk_optimize::run(ttk_optimize::OptimizeArgs {
                input: &directory,
                output: output.as_deref(),
                quality,
                keep_original,
            })
        }
        Commands::CloneRepo { repo_url, output, reset } => {
            ttk_clone_repo::run(ttk_clone_repo::CloneArgs {
                url: &repo_url,
                output: output.as_deref(),
                reset_history: reset,
            })
        }
        Commands::GitDigest { dir, since, until, last, output } => {
            ttk_git_digest::run(ttk_git_digest::GitDigestArgs {
                dir,
                since: since.as_deref().map(parse_date),
                until: until.as_deref().map(parse_date),
                last,
                output,
            })
        }
    };

    if let Err(e) = result {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}
```

- [ ] **Step 2: Build**

```bash
cd /home/ariel/term-toolkit && cargo build 2>&1
```

Expected: builds with binary at `target/debug/ttk`. 0 errors.

- [ ] **Step 3: Smoke test `--help`**

```bash
./target/debug/ttk --help
./target/debug/ttk git-digest --help
```

Expected: help text for all 5 subcommands.

- [ ] **Step 4: Commit**

```bash
git add crates/cli/src/main.rs
git commit -m "feat(cli): wire all subcommands with clap"
```

---

## Task 11: Integration smoke test

Manual end-to-end verification. No TTY automation — run by hand.

- [ ] **Step 1: Run full test suite**

```bash
cd /home/ariel/term-toolkit && cargo test 2>&1
```

Expected: all tests pass across all crates.

- [ ] **Step 2: Release build**

```bash
cargo build --release 2>&1
```

Expected: `target/release/ttk` binary, 0 errors.

- [ ] **Step 3: Test `git-digest` against this repo**

```bash
./target/release/ttk git-digest /home/ariel --last 7d
```

Expected flow:
1. Prints "Found N repositories"
2. Shows author multiselect prompt with commit counts
3. After selection: prints colored terminal output per author → repo → commit
4. Prints "Saved to /home/ariel/git-digest-<date>.md"
5. Markdown file exists and is valid

- [ ] **Step 4: Test `git-digest` wizard mode (no flags)**

```bash
./target/release/ttk git-digest /home/ariel
```

Expected: date range prompt appears first, then author prompt.

- [ ] **Step 5: Test error cases**

```bash
./target/release/ttk git-digest /nonexistent
# expected: "No git repositories found in /nonexistent" or "error: directory not found"

./target/release/ttk git-digest /home/ariel --last 3x
# expected: "error: invalid duration '3x', use 24h|7d|30d"
```

- [ ] **Step 6: Final commit**

```bash
git add -A
git commit -m "chore: rust workspace complete — 5 cmds, all tests pass"
```
