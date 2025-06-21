pub mod commands;
pub mod storage;

use chrono::{DateTime, Local};
use clap::Parser;
use commands::{Cli, Command};
use serde::{Deserialize, Serialize};
use storage::{JSONFileStorage, Storage};

use std::io;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct JournalEntry {
    title: String,
    text: String,
    tags: Vec<String>,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}

impl JournalEntry {
    pub fn new(title: String, text: String, tags: Vec<String>) -> Self {
        let created_at: DateTime<Local> = Local::now();
        let updated_at: DateTime<Local> = Local::now();

        JournalEntry {
            title,
            created_at,
            updated_at,
            tags,
            text,
        }
    }

    pub fn new_from_file(
        title: String,
        file_path: String,
        tags: Vec<String>,
    ) -> Result<Self, io::Error> {
        let text = std::fs::read_to_string(file_path)?;
        Ok(Self::new(title, text, tags))
    }
}

pub fn run() {
    let args = Cli::parse();
    let mut storage = JSONFileStorage::init("entries.json").unwrap_or_else(|_| panic!("error"));

    match args.command {
        Command::Add(args) => {
            let entry = JournalEntry {
                title: args.title,
                text: args.text,
                tags: args.tags,
                created_at: Local::now(),
                updated_at: Local::now(),
            };
            storage.save(&entry);
        }
        Command::Delete(args) => {
            let all_entries: Vec<JournalEntry> = storage
                .get_entries()
                .iter()
                .filter(|x| {
                    if args.match_title {
                        *x.title != args.title
                    } else {
                        !x.title.to_lowercase().contains(&args.title.to_lowercase())
                    }
                })
                .cloned()
                .collect();

            if all_entries.len() == storage.get_entries().len() {
                println!("Didn't delete any item");
            }

            storage.save_entries(all_entries);
        }
        Command::Search(args) => {
            fn one<T, F>(list: &Vec<T>, clo: F) -> bool
            where
                F: Fn(&T) -> bool,
            {
                if list.len() == 0 {
                    return true;
                }
                for t in list {
                    if clo(t) {
                        return true;
                    }
                }
                false
            }
            let query = args.query;
            let tags = args.tags;

            let all_entries = storage.get_entries();
            let search_results = all_entries
                .iter()
                .filter(|x| {
                    x.title.to_lowercase().contains(&query.to_lowercase())
                        && one(&tags, |tag| x.tags.contains(&tag))
                })
                .cloned()
                .collect::<Vec<JournalEntry>>();

            println!("{search_results:#?}");
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_journal_entry() {
        let entry = JournalEntry::new(
            String::from("First Journal Entry"),
            String::from("Hello Journal"),
            vec![String::from("personal")],
        );
        assert_eq!(entry.title, String::from("First Journal Entry"));
        assert_eq!(entry.text, String::from("Hello Journal"));
        assert_eq!(entry.tags, vec![String::from("personal")]);
    }
}
