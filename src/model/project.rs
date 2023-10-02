use sqlite::{Connection, Error, State, Value};

#[derive(Debug, Clone)]
pub struct Project {
    pub id: Option<i64>,
    pub name: String
}


impl std::string::ToString for Project {
    fn to_string(&self) -> String {
        self.name.to_string()
    }
}