use crate::{JournalEntry, storage::Storage};
use std::io::Error;

pub struct FileSystemStorage {}

impl Storage for FileSystemStorage {
    fn save(&mut self, _entry: &JournalEntry) -> bool {
        // Implement file system saving logic here
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
        true
    }
}
