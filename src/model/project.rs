use chrono::{DateTime, Local};
use sqlite::Statement;


#[derive(Debug, Clone)]
pub struct Project {
    pub id: Option<i64>,
    pub name: String,
    pub created_at: Option<DateTime<Local>>
}


impl std::string::ToString for Project {
    fn to_string(&self) -> String {
        self.name.to_string()
    }
}

impl From<&mut Statement<'_>> for Project {
    fn from(stmt: &mut Statement) -> Self {
        Project{
            id: Some(stmt.read::<i64, _>("id").unwrap()),
            name: stmt.read::<String, _>("name").unwrap(),
            created_at: DateTime::from_timestamp(stmt.read::<i64, _>("created_at").unwrap(), 0).map(|t|t.with_timezone(&Local))
        }
    }
}