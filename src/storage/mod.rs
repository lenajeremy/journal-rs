pub mod fs_storage;
pub mod json_storage;

use crate::JournalEntry;

pub use json_storage::JSONFileStorage;

use std::io;

pub trait Storage {
    fn save(&mut self, entry: &JournalEntry) -> bool;
    fn read(&self, title: &str) -> Result<JournalEntry, io::Error>;
}
