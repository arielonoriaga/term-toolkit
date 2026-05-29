use colored::Colorize;
use crate::git::AuthorGroups;

pub struct TermOutput;

impl TermOutput {
    pub fn print_digest(groups: &AuthorGroups) {
        let mut author_entries: Vec<(String, &String)> = groups
            .iter()
            .map(|(email, repos)| {
                let mut rp: Vec<&std::path::PathBuf> = repos.keys().collect();
                rp.sort();
                let name = rp.first()
                    .and_then(|r| repos[*r].first())
                    .map(|c| c.author_name.clone())
                    .unwrap_or_else(|| email.clone());
                (name, email)
            })
            .collect();
        author_entries.sort_by(|a, b| a.0.cmp(&b.0));

        for (name, email) in &author_entries {
            let repos = &groups[email.as_str()];

            println!("\n{}", format!("## {}", name).bold().green());

            let mut repo_paths: Vec<&std::path::PathBuf> = repos.keys().collect();
            repo_paths.sort();

            for repo in repo_paths {
                let commits = &repos[repo];
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
