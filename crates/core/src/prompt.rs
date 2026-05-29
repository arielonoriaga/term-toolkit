use chrono::{DateTime, Duration, Local, NaiveDate};
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
        let now = Local::now();
        let options = vec![
            "Last 24 hours",
            "Last 7 days",
            "Last 30 days",
            "Custom range",
        ];
        let choice = Select::new("Date range:", options)
            .prompt()
            .unwrap_or_else(|_| std::process::exit(0));
        match choice {
            "Last 24 hours" => (now - Duration::hours(24), now),
            "Last 7 days" => (now - Duration::days(7), now),
            "Last 30 days" => (now - Duration::days(30), now),
            _ => {
                let since_str = Text::new("Since (YYYY-MM-DD):")
                    .prompt()
                    .unwrap_or_else(|_| std::process::exit(0));
                let until_str = Text::new("Until (YYYY-MM-DD):")
                    .with_default(&now.format("%Y-%m-%d").to_string())
                    .prompt()
                    .unwrap_or_else(|_| std::process::exit(0));
                let since = NaiveDate::parse_from_str(&since_str, "%Y-%m-%d")
                    .expect("invalid date format, use YYYY-MM-DD")
                    .and_hms_opt(0, 0, 0)
                    .unwrap()
                    .and_local_timezone(Local)
                    .earliest()
                    .expect("ambiguous local time");
                let until = NaiveDate::parse_from_str(&until_str, "%Y-%m-%d")
                    .expect("invalid date format, use YYYY-MM-DD")
                    .and_hms_opt(23, 59, 59)
                    .unwrap()
                    .and_local_timezone(Local)
                    .latest()
                    .expect("ambiguous local time");
                (since, until)
            }
        }
    }

    pub fn select_authors(summaries: &[AuthorSummary]) -> Vec<String> {
        let options: Vec<String> = summaries
            .iter()
            .enumerate()
            .map(|(i, a)| format!("[{}] {} ({} commits, {} repos)", i, a.name, a.commit_count, a.repo_count))
            .collect();
        loop {
            let selected = MultiSelect::new("Select authors:", options.clone())
                .prompt()
                .unwrap_or_else(|_| std::process::exit(0));
            if !selected.is_empty() {
                return options
                    .iter()
                    .enumerate()
                    .filter(|(_, opt)| selected.contains(opt))
                    .map(|(i, _)| summaries[i].email.clone())
                    .collect();
            }
            eprintln!("Select at least one author.");
        }
    }
}
