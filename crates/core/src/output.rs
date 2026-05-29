use colored::Colorize;
use crate::git::AuthorGroups;

pub struct TermOutput;

impl TermOutput {
    pub fn print_digest(groups: &AuthorGroups) {
        let mut authors: Vec<&String> = groups.keys().collect();
        authors.sort();

        for email in authors {
            let repos = &groups[email];
            let name = repos
                .values()
                .flat_map(|c| c.iter())
                .next()
                .map(|c| c.author_name.as_str())
                .unwrap_or(email.as_str());

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
