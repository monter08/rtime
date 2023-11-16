use colored::Colorize;
use dialoguer::{Confirm, Input};
use sqlite::Connection;
use crate::repository::TaskRepository;
use crate::cli::Error;

pub fn delete(db: &Connection, task_id: Option<i64>) -> Result<(), Error> {
    let task_id = task_id.unwrap_or_else(||Input::new()
        .with_prompt("Task id")
        .interact_text()
        .unwrap());


    if !Confirm::new()
        .with_prompt("Do you really really want to delete?")
        .default(false)
        .show_default(false)
        .wait_for_newline(true)
        .interact()
        .unwrap() {
        return Ok(())
    }

    if TaskRepository::delete(db, task_id).is_ok() {
        println!("{}", "Task was deleted successfully".green())
    } else {
        println!("{}", "Error :(".red())
    }
    Ok(())
}