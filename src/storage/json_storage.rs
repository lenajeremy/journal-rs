use super::Storage;
use crate::JournalEntry;
use std::{
    fs,
    io::{Error, ErrorKind},
};

pub struct JSONFileStorage {
    file_path: &'static str,
    entries: Vec<JournalEntry>,
}

impl JSONFileStorage {
    /// this should create a new file and load it with an empty list of journal entries.
    /// if there is already a file, it should simply load the file and initialize the entries
    /// field.
    pub fn init(file_path: &'static str) -> Result<Self, Error> {
        if !&file_path.ends_with(".json") {
            let e = Error::new(ErrorKind::InvalidFilename, "non JSON files not allowed");
            return Err(e);
        }

        let content = std::fs::read_to_string(&file_path).unwrap_or_else(|e| {
            // create a file if the expected file doesn't exist
            _ = match e.kind() {
                ErrorKind::NotFound => fs::write(&file_path, ""),
                _ => Ok(()),
            };

            "[]".to_string()
        });

        let entries: Vec<JournalEntry> = serde_json::from_str(&content).unwrap_or(vec![]);

        Ok(JSONFileStorage {
            file_path: file_path,
            entries: entries,
        })
    }

    fn write_to_file(&self) -> bool {
        let entries_text = match serde_json::to_string(&self.entries) {
            Ok(e) => e,
            Err(_) => return false,
        };

        match fs::write(&self.file_path, entries_text) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn get_entries(&self) -> &Vec<JournalEntry> {
        &self.entries
    }
}

impl Storage for JSONFileStorage {
    fn save(&mut self, entry: &JournalEntry) -> bool {
        if let Some(_) = self.entries.iter().find(|e| e.title == entry.title) {
            return false;
        }

        self.entries.push(entry.clone());
        self.write_to_file()
    }

    fn save_entries(&mut self, entries: Vec<JournalEntry>) -> bool {
        self.entries = entries;
        self.write_to_file()
    }

    fn read(&self, title: &str) -> Result<JournalEntry, Error> {
        let res = match self.entries.iter().find(|x| {
            x.title
                .to_lowercase()
                .contains(title.to_lowercase().as_str())
        }) {
            Some(s) => Ok((*s).clone()),
            None => Err(Error::new(
                ErrorKind::NotFound,
                "Journal with that title not found",
            )),
        };
        res
    }
}

//#[cfg(test)]
//mod tests {
//    #[test]
//    fn
//}
