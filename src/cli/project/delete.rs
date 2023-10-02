use colored::Colorize;
use dialoguer::{Confirm, Input};
use sqlite::Connection;
use crate::cli::command::select_project;
use crate::repository::ProjectRepository;

pub fn delete(db: &Connection, project_id: Option<i64>) -> ()
{
    let project_id = project_id.unwrap_or_else(||{
       let project = select_project(db).expect("Error");
        ProjectRepository::find_by_name(&db, project)
            .expect("Cannot get project from DB")
            .unwrap()
            .id
            .unwrap()
    });

    if !Confirm::new()
        .with_prompt("Do you really really want to delete?")
        .default(false)
        .show_default(false)
        .wait_for_newline(true)
        .interact()
        .unwrap() {
        return ()
    }

    if ProjectRepository::delete(db, project_id).is_ok() {
        println!("{}", "Project was deleted successfully".green())
    } else {
        println!("{}", "Error :(".red())
    }
}