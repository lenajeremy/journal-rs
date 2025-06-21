use serde_json::map::Entry;

use crate::{JournalEntry, storage::Storage};
use std::{collections::HashMap, fs, io::Error};

pub struct FileSystemStorage {
    root_dir: String,
    entries: Vec<JournalEntry>,
}

impl FileSystemStorage {
    fn init(&mut self, root_dir: &'static str) -> Result<Self, Error> {
        let dir = fs::read_dir(root_dir)?;
        let entries = dir
            .map(|entry| {
                let entry = entry?;
                let content = fs::read_to_string(entry.path())?;
                let journal_entry: JournalEntry = serde_json::from_str(&content)?;
                Ok(journal_entry)
            })
            .filter(|x| x.is_ok())
            .map(|x: Result<JournalEntry, Error>| x.unwrap())
            .collect();

        let fss = FileSystemStorage {
            root_dir: root_dir.to_string(),
            entries: entries,
        };

        Ok(fss)
    }
}

impl Storage for FileSystemStorage {
    fn save(&mut self, _entry: &JournalEntry) -> bool {
        // Implement file system saving logic here
        //fs::read_dir(self.root_dir)
        true
    }

    fn read(&self, title: &str) -> Result<JournalEntry, Error> {
        // Implement file system reading logic here
        Ok(JournalEntry::new(
            title.to_string(),
            String::new(),
            Vec::new(),
        ))
    }

    fn save_entries(&mut self, entries: Vec<JournalEntry>) -> bool {
        let mut safe_entries: HashMap<String, bool> = HashMap::new();
        entries.iter().for_each(|entry| {
            safe_entries.insert(entry.title.clone(), true);
        });

        if let Ok(dir) = fs::read_dir(&self.root_dir) {
            return dir.all(|entry| {
                let entry = match entry {
                    Ok(e) => e,
                    Err(_) => return false,
                };

                match fs::remove_file(entry.path()) {
                    Ok(_) => true,
                    Err(_) => false,
                }
            });
        } else {
            return false;
        }
    }
}
