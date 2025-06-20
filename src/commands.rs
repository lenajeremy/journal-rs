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
    /// Title of entry(ies) to delet sf
    #[arg(long)]
    pub title: String,

    #[arg(long)]
    pub match_title: bool,
}

#[derive(Args, Debug)]
pub struct AddArgs {
    /// The text of the journal entry.
    #[arg(long)]
    pub text: String,

    /// The title of the journal entry.
    #[arg(long)]
    pub title: String,

    /// The categories this entry contains.
    #[arg(long)]
    pub tags: Vec<String>,
    ///// The date of the journal entry.
    //pub date: Option<String>,
}
