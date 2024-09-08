use clap::Parser;
use tasks::{
    cli::{Cli, Commands, ScopeCommands},
    configuration::Settings,
    domain::{NewTask, Scope, Task},
    scopes,
    startup::{ensure_initialized, Application},
    storage::{self, Folder},
    tabular::get_tasks_table,
    tasks::{add_task, complete_task, delete_task, list_tasks},
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let storage_folder = {
        let mut storage_folder = storage::get_folder_path(Folder::Local);
        storage_folder.push("tasks");
        storage_folder.set_extension("db");
        storage_folder
    };
    let app_settings = Settings::new(storage_folder);

    ensure_initialized(&app_settings).await?;
    let app = Application::build(app_settings).await?;
    match args.get_command() {
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
            let builder = get_tasks_table(120);
            let table = builder.build().unwrap();
            table.print(tasks);
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
