use chrono::{DateTime, Local, NaiveDate};
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
        /// Keep original file (default: true). Use --keep-original=false to overwrite
        #[arg(long, default_value_t = true, action = clap::ArgAction::Set)]
        keep_original: bool,
    },
    /// Clone a repo and optionally reset history
    CloneRepo {
        repo_url: String,
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Reset git history after clone (default: true). Use --reset=false to keep history
        #[arg(long, default_value_t = true, action = clap::ArgAction::Set)]
        reset: bool,
    },
    /// Summarize commits across repos in a date range
    GitDigest {
        dir: PathBuf,
        /// Date ISO 8601, e.g. 2026-05-01
        #[arg(long)]
        since: Option<String>,
        /// Date ISO 8601, e.g. 2026-05-28
        #[arg(long)]
        until: Option<String>,
        /// Shorthand: 24h | 7d | 30d
        #[arg(long)]
        last: Option<String>,
        /// Output directory for markdown file
        #[arg(long)]
        output_dir: Option<PathBuf>,
    },
}

fn parse_since_date(s: &str) -> Result<DateTime<Local>, String> {
    parse_date_at(s, 0, 0, 0)
}

fn parse_until_date(s: &str) -> Result<DateTime<Local>, String> {
    parse_date_at(s, 23, 59, 59)
}

fn parse_date_at(s: &str, h: u32, m: u32, sec: u32) -> Result<DateTime<Local>, String> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|_| format!("invalid date '{}', use YYYY-MM-DD", s))?
        .and_hms_opt(h, m, sec)
        .unwrap()
        .and_local_timezone(Local)
        .earliest()
        .ok_or_else(|| format!("ambiguous local time for '{}'", s))
}

fn run_command(cli: Cli) -> Result<(), String> {
    match cli.command {
        Commands::Deleter { directory, even } => {
            ttk_deleter::run(&directory, even)
        }
        Commands::Rename { new_name, directory } => {
            ttk_rename::run(&directory, &new_name)
        }
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
        Commands::GitDigest { dir, since, until, last, output_dir } => {
            ttk_git_digest::run(ttk_git_digest::GitDigestArgs {
                dir,
                since: since.as_deref().map(parse_since_date).transpose()?,
                until: until.as_deref().map(parse_until_date).transpose()?,
                last,
                output_dir,
            })
        }
    }
}

fn main() {
    if let Err(e) = run_command(Cli::parse()) {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}
