use clap::{command, Parser, Subcommand};
use tasks::{
    configuration::Settings,
    domain::{NewTask, Scope, Task},
    scopes,
    startup::{ensure_initialized, Application},
    storage::{self, Folder},
    tasks::{add_task, complete_task, delete_task, list_tasks},
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
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

#[derive(Subcommand, Debug)]
enum ScopeCommands {
    List,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let storage_folder = {
        let mut storage_folder = storage::get_folder_path(Folder::Local);
        storage_folder.push("tasks");
        storage_folder.set_extension("db");
        storage_folder
    };
    let app_settings = Settings::new(storage_folder);

    ensure_initialized(&app_settings).await?;
    let app = Application::build(app_settings).await?;
    match args.command {
        Commands::Add { description, scope } => {
            let input = NewTask {
                description,
                scope: scope.map(Scope::new),
            };
            add_task(&app.pool, &app.generator, input).await?;
        }
        Commands::List { scope } => {
            let tasks = list_tasks(&app.pool, scope.map(Scope::new)).await?;
            let tasks: Vec<Task> = tasks.into_iter().filter_map(|x| x.ok()).collect();
            println!("Completed\tID\tDescription\tScope\tCreated at");
            for task in tasks {
                println!(
                    "- [{}]\t\t{}\t{}\t{}\t{}",
                    task.completed_at.map_or(" ", |_| "x"),
                    task.id,
                    task.description,
                    task.scope.as_ref().map_or("None", |s| s.as_ref()),
                    task.created_at.format("%Y-%m-%d %H:%M:%S")
                );
            }
        }
        Commands::Complete { id } => {
            let success = complete_task(&app.pool, id).await?;
            if success {
                println!("Successfully completed task with id {}", id)
            } else {
                println!("Task with id {} not found", id)
            }
        }
        Commands::Delete { id } => {
            let success = delete_task(&app.pool, id).await?;
            if success {
                println!("Successfully deleted task with id {}", id)
            } else {
                println!("Task with id {} not found", id)
            }
        }
        Commands::Scope { action } => match action {
            ScopeCommands::List => {
                let scopes = scopes::list(&app.pool).await?;
                println!("The following scopes have been found:");
                for scope in scopes {
                    println!("{}", scope.as_ref())
                }
            }
        },
    }

    Ok(())
}
