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
                tags: vec!["personal".to_string()],
                created_at: Local::now(),
                updated_at: Local::now(),
            };
            storage.save(&entry);

            println!(
                "Entry = {:#?}\nEntries = {:#?}",
                entry,
                storage.get_entries()
            );
        }
        Command::Delete(args) => {
            println!("Deleting {}", args.date);
        }
        Command::Search(args) => {
            let query = args.query;
            let search_range = args.search_range;
            println!("{}, {:?}", query, search_range)
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
