use crate::model::Project;
use chrono::{DateTime, Local};
use sqlite::Statement;

#[derive(Debug, Clone)]
pub struct Task {
    pub id: Option<i64>,
    pub project: Project,
    pub name: String,
    pub time: i64,
    pub created_at: Option<DateTime<Local>>
}

impl std::string::ToString for Task {
    fn to_string(&self) -> String {
        self.name.to_string()
    }
}

impl From<&mut Statement<'_>> for Task {
    fn from(stmt: &mut Statement) -> Self {
        Task{
            id: Some(stmt.read::<i64, _>("task_id").unwrap()),
            project: Project {
                id: Some(stmt.read::<i64, _>("project_id").unwrap()),
                name: stmt.read::<String, _>("project_name").unwrap(),
                created_at: DateTime::from_timestamp(stmt.read::<i64, _>("project_created_at").unwrap(), 0).map(|t|t.with_timezone(&Local)),
            },
            name: stmt.read::<String, _>("task_name").unwrap(),
            time: stmt.read::<i64, _>("task_time").unwrap(),
            created_at: DateTime::from_timestamp(stmt.read::<i64, _>("task_created_at").unwrap(), 0).map(|t|t.with_timezone(&Local)),
        }
    }
}