use clap::{command, CommandFactory, Parser, Subcommand};
use tasks::{
    configuration::Settings,
    domain::Task,
    startup::{ensure_db_created, Application},
    tasks::{add_task, complete_task, delete_task, list_tasks},
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Adds new task
    Add {
        /// Task description
        description: String,
    },
    /// List tasks
    List,
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
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let app_settings = Settings::new("tasks.db".into());

    if let Some(command) = args.command {
        ensure_db_created(&app_settings).await?;
        let app = Application::build(app_settings).await?;
        match command {
            Command::Add { description } => {
                add_task(&app.pool, &app.generator, description).await?;
            }
            Command::List => {
                let tasks = list_tasks(&app.pool).await?;
                let tasks: Vec<Task> = tasks.into_iter().filter_map(|x| x.ok()).collect();
                for task in tasks {
                    println!(
                        "- [{}]\t{}\t{}\t{}",
                        task.completed_at.map_or(" ", |_| "X"),
                        task.id,
                        task.description,
                        format!("{}", task.created_at.format("%Y-%m-%d %H:%M:%S"))
                    );
                }
            }
            Command::Complete { id } => {
                let success = complete_task(&app.pool, id).await?;
                if success {
                    println!("Successfully completed task with id {}", id)
                } else {
                    println!("Task with id {} not found", id)
                }
            }
            Command::Delete { id } => {
                let success = delete_task(&app.pool, id).await?;
                if success {
                    println!("Successfully deleted task with id {}", id)
                } else {
                    println!("Task with id {} not found", id)
                }
            }
        }
    } else {
        let mut cmd = Args::command();
        let _ = cmd.print_help();
    }
    Ok(())
}
