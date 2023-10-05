use sqlite::{Connection, Error, State, Value};
use crate::model::Project;
#[derive(Debug, Clone)]
pub struct Task {
    pub id: Option<i64>,
    pub project: Project,
    pub name: String,
    pub time: i64,
}

impl std::string::ToString for Task {
    fn to_string(&self) -> String {
        self.name.to_string()
    }
}