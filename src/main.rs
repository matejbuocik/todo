mod cli;
mod config;
mod db;
use clap::{Parser, Subcommand};

/// Todo List
#[derive(Parser, Debug)]
#[command(name = "todo")]
#[command(version, about = "TODO List", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Add notes
    #[command(arg_required_else_help = true)]
    Add {
        /// Notes to add
        notes: Vec<String>,
    },

    /// Set notes as done
    #[command(arg_required_else_help = true)]
    Done {
        /// Notes to set as done
        notes: Vec<u64>,
    },

    /// Remove notes
    #[command(arg_required_else_help = true)]
    Remove {
        /// Notes to remove
        notes: Vec<u64>,
    },

    /// Edit notes
    #[command(arg_required_else_help = true)]
    Edit {
        /// Notes to edit
        notes: Vec<u64>,
    },

    /// List notes
    List,
}

fn main() {
    let args = Cli::parse();
    if let Err(e) = cli::run(&args) {
        eprintln!("todo - error: {:?}", e);
        std::process::exit(1);
    };
}
