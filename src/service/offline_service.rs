use crate::service::database_api::TasksDatabase;
use console::style;

pub fn show_stats(tasks_database: TasksDatabase, shrink: bool) -> anyhow::Result<()> {
    let tasks = tasks_database.get_data();
    if tasks.is_err() {
        println!("logged_out");
        return Ok(());
    };
    let mut incomplete = 0;
    let mut complete = 0;
    let total = &tasks.as_ref().unwrap().len();
    for task in &tasks.unwrap() {
        if task.status == "needsAction" {
            incomplete += 1;
        } else {
            complete += 1;
        }
    }
    let (_, task_list_name) = tasks_database.get_default_tasklist()?;
    if shrink {
        println!("{}_{}_{}_{}", task_list_name, complete, incomplete, total);
    } else {
        println!(
            "{}\n{} {}\n{} {}\n{} {}",
            style(task_list_name).yellow().bold().underlined(),
            style("Completed Tasks: ").green(),
            style(complete).green(),
            style("Incomplete Tasks: ").red(),
            style(incomplete).red(),
            style("Total Tasks: ").cyan(),
            total
        );
    }
    Ok(())
}
