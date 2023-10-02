use std::{env, fs};
use std::fs::File;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use colored::Colorize;
use sqlite::Connection;
use crate::cli::command::select_project;
use crate::model::Project;
use crate::repository::ProjectRepository;

pub fn init(db: &Connection, project: Option<String>) -> (){
    let git = Path::new(".git");
    if  !git.exists() {
        println!("{}", "Git is not initialized here".red());
        return ()
    }

    let hook = git.join("hooks/post-commit");
    if hook.exists() {
        println!("{}", "Hook is already initialized here".red());
        return ()
    }

    let input_project = project.unwrap_or_else(|| select_project(db).expect("Error"));
    let project = ProjectRepository::find_by_name(db, input_project.clone())
        .expect("Error get project @todo");

    let bin_path = env::current_exe().expect("failed to get current exe path");
    match project {
        None => {
            println!("{} {} {}", "Project".red(), input_project.to_string().red(), "not exists");
        }
        Some(project) => {
            let mut f = File::create(hook.clone()).expect("Failed to create file");
            fs::set_permissions(hook, fs::Permissions::from_mode(0o755)).unwrap();
            f.write_all(
                format!("#!/bin/bash\nmsg=`git log -1 --pretty=%B`\n{} task create \"{}\" --name=\"$msg\" ", bin_path.display(), project.name)
                    .as_bytes())
                .expect("TODO: panic message");
            println!("{}", "Init done.".green())
        }
    }
}