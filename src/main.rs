extern crate alloc;
extern crate core;

use crate::db::DB;
use crate::model::{Project, Task};
use crate::repository::{ProjectRepository, TaskRepository};
use dialoguer::{Input, Select};
use sqlite::Connection;
use clap::{Parser, Subcommand};

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

    cli::start(&db);
}

