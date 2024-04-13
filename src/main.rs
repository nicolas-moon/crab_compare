use std::{error::Error, fs};

use clap::{Parser, ValueEnum};
use git2::{BranchType, Repository};
use prettytable::{Cell, row, Row, Table};
use serde::Deserialize;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Deserialize, ValueEnum)]
enum OutputFormat {
    Markdown,
    Table,
}

#[derive(Deserialize, Debug)]
struct Config {
    repo_path: String,
    target_branches: Vec<String>,
    output_table: OutputFormat,
}

impl Config {
    fn new() -> Self {
        Config {
            repo_path: ".".to_string(),
            target_branches: vec!["main".to_string()],
            output_table: OutputFormat::Table,
        }
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// path to the config toml for the compare run
    #[clap(short, long, default_value = "crab.toml")]
    config: String,

    /// Path to the git repository
    #[clap(short, long)]
    repo_path: Option<String>,

    /// target branch to compare to
    #[clap(long, use_value_delimiter = true)]
    target_branches: Option<Vec<String>>,

    /// Enable output in table format
    #[clap(long, default_value = "table")]
    output_table: Option<OutputFormat>,
}

impl Args {
    fn merge_with_config(self, config: Config) -> Config {
        println!("Merging args with config: {:?}", self.output_table.unwrap());
        Config {
            repo_path: self.repo_path.unwrap_or(config.repo_path),
            target_branches: self.target_branches.unwrap_or(config.target_branches),
            output_table: self.output_table.unwrap_or(config.output_table),
        }
    }
}

fn main() {
    let args = Args::parse();
    let config = if fs::metadata(&args.config).is_ok() {
        let content = fs::read_to_string(&args.config).expect("failed to read config file");
        let mut config: Config = toml::from_str(&content).expect("failed to parse config file");
        if args.repo_path.is_some() || args.target_branches.is_some() {
            config = args.merge_with_config(config);
        }
        config
    } else {
        Config::new()
    };

    match compare_with_all_branches(&config) {
        Ok(_) => println!("Comparison complete."),
        Err(e) => println!("Error: {}", e),
    }
}

fn compare_with_all_branches(config: &Config) -> Result<(), Box<dyn Error>> {
    let repo = Repository::open(&config.repo_path)?;

    let branches = repo.branches(Some(BranchType::Local))?;
    let target_branch_ids = config
        .target_branches
        .iter()
        .map(|branch_name| {
            repo.find_branch(branch_name, BranchType::Local)
                .and_then(|branch| {
                    branch
                        .get()
                        .peel_to_commit()
                        .map(|c| (branch_name.clone(), c.id()))
                })
        })
        .collect::<Result<Vec<_>, git2::Error>>()?;

    let mut results = vec![
        "| Branch Name | Commit Hash | Target Branch |".to_string(),
        "| ----------- | ----------- | ------------- |".to_string(),
    ];

    let mut table = Table::new();
    table.add_row(row!["Branch Name", "Commit Hash", "Target Branch"]);

    for branch in branches {
        let (branch, _) = branch?;
        let branch_name = branch.name()?.unwrap_or_default().to_string();
        if !config.target_branches.contains(&branch_name) {
            let commit = branch.get().peel_to_commit()?;
            let commit_id = commit.id();
            for (target_branch, target_id) in &target_branch_ids {
                if *target_id == commit_id {
                    match config.output_table {
                        OutputFormat::Markdown => results.push(format!("| {} | {} | {} |", branch_name, commit_id, target_branch)),
                        OutputFormat::Table => {
                            table.add_row(Row::new(vec![
                                Cell::new(&branch_name),
                                Cell::new(&commit_id.to_string()),
                                Cell::new(&target_branch),
                            ]));
                        }
                    }
                }
            }
        }
    }

    if config.output_table == OutputFormat::Table {
        table.printstd();
    }

    if config.output_table == OutputFormat::Markdown {
        for line in results {
            println!("{}", line)
        }
    }

    Ok(())
}
