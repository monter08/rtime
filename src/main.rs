extern crate alloc;
extern crate core;
use crate::db::DB;
use crate::repository::{ProjectRepository, TaskRepository};
use colored::Colorize;
mod db;
mod repository;
mod model;
mod cli;

fn main() {
    // Connecting SQLite
    let db = DB{filename: DB::path().expect("Path error") }.connect();

    // Init databases
    TaskRepository::init(&db);
    ProjectRepository::init(&db);

    match cli::start(&db) {
        Ok(_) => {}
        Err(err) => {
            println!("{}", err.to_string().red())
        }
    }
}

