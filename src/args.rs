use clap::{Args, Parser, Subcommand};



#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct DaysArgs
{
    #[clap(subcommand)]
    pub entity_type: EntityType, 
}

#[derive(Subcommand, Debug)]
pub enum EntityType
{
    /// See events in the file
    List(ListArgs),

    /// Add events to the file
    Add(AddArgs),

    /// Delete events from the file
    Delete(ListArgs)
}


/// All arugments for the list and delete commands
#[derive(Parser, Debug)]
pub struct ListArgs {
    /// All events today
    #[arg(long, default_value_t = false)]
    pub today: bool,
    
    /// All events before a date
    #[arg(long, default_value_t = String::from(""))]
    pub before_date: String,

    /// All events after a date
    #[arg(long, default_value_t = String::from(""))]
    pub after_date: String,

    /// All events on a specific date
    #[arg(long, default_value_t = String::from(""))]
    pub date: String,

    /// All events in a specific category
    #[arg(long, default_value_t = String::from(""))]
    pub category: String,

    /// All events in a specific categories
    #[arg(long, default_value_t = String::from(""))]
    pub categories: String,

    /// All events with a specific description
    #[arg(long, default_value_t = String::from(""))]
    pub description: String,

    /// All events without a category
    #[arg(long, default_value_t = false)]
    pub no_category: bool,

    /// All events without a description
    #[arg(long, default_value_t = false)]
    pub no_description: bool,

    /// Opposite of what you wrote
    #[arg(long, default_value_t = false)]
    pub exclude: bool,

    /// All events
    #[arg(long, default_value_t = false)]
    pub all: bool,

    /// Don't save anything
    #[arg(long, default_value_t = false)]
    pub dry_run: bool,
    
}

/// All arugments for the add command
#[derive(Parser, Debug)]
pub struct AddArgs
{
    /// The date of the event
    #[arg(long, default_value_t = String::from(""))]
    pub date: String,

    /// The category of the event
    #[arg(long, default_value_t = String::from(""))]
    pub category: String,

    /// The description of the event
    #[arg(long, default_value_t = String::from(""))]
    pub description: String,

    /// Don't save anything
    #[arg(long, default_value_t = false)]
    pub dry_run: bool,
}