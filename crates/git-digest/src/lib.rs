use chrono::{DateTime, Duration, Local};
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

pub fn resolve_last(last: &str) -> Result<(DateTime<Local>, DateTime<Local>), String> {
    let now = Local::now();
    let since = match last {
        "24h" => now - Duration::hours(24),
        "7d"  => now - Duration::days(7),
        "30d" => now - Duration::days(30),
        other => return Err(format!("invalid duration '{}', use 24h|7d|30d", other)),
    };
    Ok((since, now))
}

pub fn run(args: GitDigestArgs) -> Result<(), String> {
    // Step 1: Validate + scan
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
        // No TTY guard: if stdin is not a terminal and no flags given, error
        #[cfg(unix)]
        {
            use std::os::unix::io::AsRawFd;
            let is_tty = unsafe { libc::isatty(std::io::stdin().as_raw_fd()) } != 0;
            if !is_tty {
                return Err(
                    "no TTY detected and no date flags given — use --last or --since/--until"
                        .to_string(),
                );
            }
        }
        Prompt::date_range()
    };

    if since > until {
        return Err("--since must be before --until".to_string());
    }

    // Step 2: git log with optional progress bar
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
        if let Some(ref bar) = pb {
            bar.inc(1);
        }
    }
    if let Some(bar) = pb {
        bar.finish_and_clear();
    }

    if all_commits.is_empty() {
        println!("No commits found in the selected period.");
        return Ok(());
    }

    // Step 3: Build author summaries
    let mut by_author: HashMap<String, Vec<&Commit>> = HashMap::new();
    for commit in &all_commits {
        by_author.entry(commit.email.clone()).or_default().push(commit);
    }
    let mut summaries: Vec<AuthorSummary> = by_author
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
    summaries.sort_by(|a, b| a.name.cmp(&b.name));

    // Step 4: Author selection
    let selected_emails = Prompt::select_authors(&summaries);

    // Step 5: Group + render
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

    TermOutput::print_digest(&groups);

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
    fn test_resolve_last_30d() {
        let (since, until) = resolve_last("30d").unwrap();
        let diff = until - since;
        assert_eq!(diff.num_days(), 30);
    }

    #[test]
    fn test_resolve_last_invalid() {
        assert!(resolve_last("3x").is_err());
        assert!(resolve_last("").is_err());
        assert!(resolve_last("24H").is_err()); // case-sensitive
    }
}
