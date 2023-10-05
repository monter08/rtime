use dialoguer::{Input};
use sqlite::Connection;
use crate::repository::{ProjectRepository, TaskRepository};
use colored::Colorize;
use crate::model::{Task};
use crate::cli::command::select_project;
pub fn create(
    db: &Connection,
    name: Option<String>,
    project: Option<String>,
    time: Option<String>
) -> () {
    let input_project = project.unwrap_or_else(|| select_project(db).expect("Error"));
    let project = ProjectRepository::find_by_name(db, input_project.clone())
        .expect("Error get project @todo");

    match project {
        None => {
            println!("{} {} {}", "Project:".red(), input_project.red().bold(), "does not exists.".red());
        }
        Some(project) => {
            let name = name.unwrap_or_else(||Input::new()
                .with_prompt("Task name")
                .interact_text()
                .unwrap());


            let mut minutes = 0;
            let mut input_time;
            loop {
                input_time = time.clone().unwrap_or_else(|| Input::new()
                    .with_prompt("How much time have you burned? (ex: 1h 30m)")
                    .interact_text()
                    .unwrap());

                match to_minutes(&input_time) {
                    Ok(min) => {
                        minutes = min;
                        break
                    }
                    Err(err) => {
                        println!("{}", err.bold())
                    }
                }
            }

            TaskRepository::insert(db, Task {
                id: None,
                project: project.clone(),
                name: name.clone(),
                time: minutes as i64
            }).expect("Failed to insert task into the repository");

            println!("{} {} {} {}", "Ok! Task name:".green(), name.green().bold(), "was added with time:".green(), input_time.green().bold());
            ()
        }
    }
}


fn to_minutes(time_str: &str) -> Result<i32, &'static str> {
    let mut total_minutes = 0;

    for part in time_str.split_whitespace() {
        if part.ends_with('h') {
            if let Ok(hours) = part.trim_end_matches('h').parse::<i32>() {
                total_minutes += hours * 60;
            } else {
                return Err("Failed to parse hours");
            }
        } else if part.ends_with('m') {
            if let Ok(minutes) = part.trim_end_matches('m').parse::<i32>() {
                total_minutes += minutes;
            } else {
                return Err("Failed to parse minutes");
            }
        } else {
            return Err("Invalid format");
        }
    }

    Ok(total_minutes)
}
