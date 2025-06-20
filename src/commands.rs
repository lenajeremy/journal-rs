use clap::{Args, Parser, Subcommand};

/// Simple journal app
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Add journal entries.
    Add(AddArgs),

    /// Delete journal entries.
    Delete(DeleteArgs),

    /// Search journal entries.
    Search(SearchArgs),
}

#[derive(Debug, Args)]
pub struct SearchArgs {
    pub query: String,
    pub search_range: Vec<String>,
}

#[derive(Args, Debug)]
pub struct DeleteArgs {
    // what date to delete
    pub date: String,
}

#[derive(Args, Debug)]
pub struct AddArgs {
    /// The text of the journal entry.
    pub text: String,

    /// The title of the journal entry.
    pub title: String,

    /// The categories this entry contains.
    pub tags: Vec<String>,
    ///// The date of the journal entry.
    //pub date: Option<String>,
}
