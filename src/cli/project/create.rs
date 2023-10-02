use dialoguer::{Input, Select};
use sqlite::{Connection, State};
use crate::repository::{ProjectRepository, TaskRepository};
use colored::Colorize;
use crate::cli::Error;
use crate::model::Project;

pub fn create(
    db: &Connection,
    name: Option<String>
) -> () {
    let name = name.unwrap_or(Input::new()
        .with_prompt("Project name")
        .interact_text()
        .unwrap());


    let create = ProjectRepository::insert(db, Project {
        id: None,
        name: name.to_string(),
    });

    if create.is_ok() {
        println!("{}", "Project added.".green());
    }
}