use std::fs;
use std::path::Path;
use colored::Colorize;
use crate::cli::Error;
pub fn clean() -> Result<(), Error>{
    let file = Path::new(".git/hooks/post-commit");
    if file.exists() {
        fs::remove_file(file).expect("TODO: panic message");
        println!("{}", "Hook was removed.".green());
        return Ok(())
    } else {
        println!("{}", "Hook is not initialized here.".red());
    }
    Ok(())
}