use dialoguer::{Input};
use sqlite::Connection;
use crate::repository::{TaskRepository};
use colored::Colorize;
use crate::model::{Task};
use crate::cli::command::select_project;
use crate::cli::Error;

pub fn create(
    db: &Connection,
    name: Option<String>,
    project: Option<String>,
    time: Option<String>
) -> Result<(), Error> {
    let project = select_project(db, project)?;

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
                println!("{}", err.red().bold())
            }
        }
    }

    let _ = TaskRepository::insert(db, Task {
        id: None,
        project: project.clone(),
        name: name.clone(),
        time: minutes,
        created_at: None
    })?;

    println!("{}{}{} {}", "Ok! Task \"".green(), name.green().bold(),"\" was added with time:".green(), input_time.green().bold());
    Ok(())
}


fn to_minutes(time_str: &str) -> Result<i64, &'static str> {
    let mut total_minutes = 0;

    for part in time_str.split_whitespace() {
        if part.ends_with('h') {
            if let Ok(hours) = part.trim_end_matches('h').parse::<i64>() {
                total_minutes += hours * 60;
            } else {
                return Err("Failed to parse hours");
            }
        } else if part.ends_with('m') {
            if let Ok(minutes) = part.trim_end_matches('m').parse::<i64>() {
                total_minutes += minutes;
            } else {
                return Err("Failed to parse minutes");
            }
        } else {
            return Err("Invalid format, try ex: 1h 30m");
        }
    }

    Ok(total_minutes)
}
