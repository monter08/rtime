use chrono::{Datelike, DateTime, Local};
use sqlite::Connection;
use crate::repository::{ProjectRepository};
use cli_table::{Cell, Style, Table};
use crate::cli::command::minutes_display;
use crate::cli::Error;
pub fn list(db: &Connection) -> Result<(), Error> {
    let from_month = Local::now().with_day(1).unwrap();

    let vec = ProjectRepository::list(&db)?.into_iter().map(|project|
        vec![
            project.id.clone().map(|id| id.to_string()).expect("0"),
            project.name.clone(),
            project.created_at.map(|date| date.format("%Y-%m-%d %H:%M:%S").to_string()).unwrap_or("-".to_string()),
            minutes_display(ProjectRepository::sum_work_time(&db, project.id.unwrap(), from_month.clone()).unwrap())
        ]
    );
    let table = vec.table()
        .title(vec![
            "ID".cell().bold(true),
            "Name".cell().bold(true),
            "Created At".cell().bold(true),
            "Total in the current month".cell().bold(true),
        ])
        .bold(true);

    let table_display = table.display().unwrap();

    println!("{}", table_display);
    Ok(())
}