use dialoguer::{Input};
use sqlite::{Connection};
use crate::repository::{ProjectRepository};
use colored::Colorize;
use crate::model::Project;
use crate::cli::Error;

pub fn create(
    db: &Connection,
    name: Option<String>
) -> Result<(), Error> {
    let name = name.unwrap_or(Input::new()
        .with_prompt("Project name")
        .interact_text()
        .unwrap());


    let create = ProjectRepository::insert(db, Project {
        id: None,
        name: name.to_string(),
        created_at: None
    });

    if create.is_ok() {
        println!("{}", "Project added.".green());
    }
    Ok(())
}