use std::{env, fs};
use std::fs::File;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path};
use colored::Colorize;
use sqlite::Connection;
use crate::cli::command::select_project;


use crate::cli::Error;
pub fn init(db: &Connection, project: Option<String>) -> Result<(), Error>{
    let git = Path::new(".git");
    if  !git.exists() {
        println!("{}", "Git is not initialized here".red());
        return Ok(())
    }

    let hook = git.join("hooks/post-commit");
    if hook.exists() {
        println!("{}", "Hook is already initialized here".red());
        return Ok(())
    }


    let project = select_project(db, project)?;
    let bin_path = env::current_exe().expect("failed to get current exe path");

    let mut f = File::create(hook.clone()).expect("Failed to create file");
    fs::set_permissions(hook, fs::Permissions::from_mode(0o755)).unwrap();
    f.write_all(
        format!("#!/bin/bash\nmsg=`git log -1 --pretty=%B`\n{} task create \"{}\" --name=\"$msg\" ", bin_path.display(), project.name)
            .as_bytes())
        .expect("TODO: panic message");
    println!("{}", "Init done.".green());

    Ok(())
}