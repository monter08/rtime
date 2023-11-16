use colored::Colorize;
use dialoguer::{Confirm};
use sqlite::Connection;
use crate::cli::command::select_project;
use crate::repository::ProjectRepository;
use crate::cli::Error;
pub fn delete(db: &Connection, project: Option<String>) -> Result<(), Error>
{
    let project = select_project(db, project)?;

    if !Confirm::new()
        .with_prompt("Do you really really want to delete?")
        .default(false)
        .show_default(false)
        .wait_for_newline(true)
        .interact()
        .unwrap() {
        return Ok(())
    }

    if ProjectRepository::delete(db, project.id.unwrap()).is_ok() {
        println!("{}", "Project was deleted successfully".green())
    } else {
        println!("{}", "Error :(".red())
    }

    Ok(())
}