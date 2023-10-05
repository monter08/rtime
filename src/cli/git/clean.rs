use std::fs;
use std::path::Path;
use colored::Colorize;

pub fn clean() -> (){
    let file = Path::new(".git/hooks/post-commit");
    if file.exists() {
        fs::remove_file(file).expect("TODO: panic message");
        println!("{}", "Hook was removed.".green());
        return ()
    } else {
        println!("{}", "Hook is not initialized here.".red());
    }
}