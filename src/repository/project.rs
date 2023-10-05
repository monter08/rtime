use sqlite::{Connection, Error, State, Value};
use crate::model::{Project, Task};

pub struct ProjectRepository {
}

impl ProjectRepository {
    pub fn init(db: &Connection)  {
        db.execute("CREATE TABLE IF NOT EXISTS projects (id INTEGER, name TEXT,PRIMARY KEY(id AUTOINCREMENT))").expect("Cannot create table projects");
    }

    pub fn find(db: &Connection, id: i64) -> Result<Option<Project>, Error>{
        let mut stmt = db.prepare("SELECT id, name from projects where id = :id")?;
        stmt.bind::<&[(_, Value)]>(&[
            (":id", id.into()),
        ])?;
        let mut project = None;
        while let Ok(State::Row) = stmt.next() {
            project = Some(Project{
                id: Some(stmt.read::<i64, _>("id").unwrap()),
                name: stmt.read::<String, _>("name").unwrap(),
            });
            break;
        }

        Ok(project)
    }

    pub fn find_by_name(db: &Connection, name: String) -> Result<Option<Project>, Error>{
        let mut stmt = db.prepare("SELECT id, name from projects where name = :name")?;
        stmt.bind::<&[(_, Value)]>(&[
            (":name", name.into()),
        ])?;
        let mut project = None;
        while let Ok(State::Row) = stmt.next() {
            project = Some(Project{
                id: Some(stmt.read::<i64, _>("id").unwrap()),
                name: stmt.read::<String, _>("name").unwrap(),
            });
            break;
        }

        Ok(project)
    }

    pub fn list(db: &Connection) -> Result<Vec<Project>, Error> {
        let mut stmt = db.prepare("SELECT id, name from projects")?;
        let mut projects = vec![];
        while let Ok(State::Row) = stmt.next() {
            projects.push(Project{
                id: Some(stmt.read::<i64, _>("id").unwrap()),
                name: stmt.read::<String, _>("name").unwrap(),
            });
        }
        Ok(projects)
    }

    pub fn insert(db: &Connection, project: Project) -> sqlite::Result<State> {
        let mut stmt = db.prepare("INSERT INTO projects (id, name) VALUES (null, :name)").unwrap();
        stmt.bind::<&[(_, Value)]>(&[
            (":name", project.name.clone().into()),
        ]).expect("TODO: panic message");

        stmt.next()
    }
    pub fn delete(db: &Connection, task_id: i64) -> sqlite::Result<State> {
        let mut stmt = db.prepare("DELETE from tasks where project_id = :id").unwrap();
        stmt.bind::<&[(_, Value)]>(&[
            (":id", task_id.into()),
        ]).expect("TODO: panic message");

        let mut stmt = db.prepare("DELETE from projects where id = :id").unwrap();
        stmt.bind::<&[(_, Value)]>(&[
            (":id", task_id.into()),
        ]).expect("TODO: panic message");

        stmt.next()
    }
}