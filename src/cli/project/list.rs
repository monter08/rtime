use sqlite::Connection;
use crate::repository::{ProjectRepository, TaskRepository};
use cli_table::{format::Justify, print_stdout, Cell, Style, Table};

pub fn list(db: &Connection) -> () {
    let vec = ProjectRepository::list(&db).expect("Error").into_iter().map(|project|
        vec![
            project.id.clone().map(|id| id.to_string()).expect("0"),
            project.name.clone(),
        ]
    );
    let table = vec.table()
        .title(vec![
            "ID".cell().bold(true),
            "Name".cell().bold(true),
        ])
        .bold(true);

    let table_display = table.display().unwrap();

    println!("{}", table_display);
}