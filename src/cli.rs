use clap::{command, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Adds new task
    Add {
        /// Task description
        description: String,
        /// Task scope
        #[arg(long, short)]
        scope: Option<String>,
    },
    /// List tasks
    #[clap(visible_alias = "ls")]
    List {
        /// Scope filter
        #[arg(long, short)]
        scope: Option<String>,
    },
    /// Toggles task completion
    Complete {
        #[arg(value_name = "TASK_ID")]
        id: u32,
    },
    /// Deletes task
    Delete {
        #[arg(value_name = "TASK_ID")]
        id: u32,
    },
    /// Scope management actions
    Scope {
        #[command(subcommand)]
        action: ScopeCommands,
    },
}

#[derive(Subcommand, Debug, Clone)]
pub enum ScopeCommands {
    List,
}

impl Cli {
    pub fn get_command(&self) -> Commands {
        self.command
            .clone()
            .unwrap_or(Commands::List { scope: None })
    }
}
