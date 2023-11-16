use chrono::{DateTime, Local};
use sqlite::{Connection, Error, State, Value};
use crate::model::{Project};

pub struct ProjectRepository {
}

impl ProjectRepository {
    pub fn init(db: &Connection)  {
        db.execute("CREATE TABLE IF NOT EXISTS projects (id INTEGER, name TEXT,created_at datetime DEFAULT CURRENT_TIMESTAMP,PRIMARY KEY(id AUTOINCREMENT))").expect("Cannot create table projects");
    }

    /*
    pub fn find(db: &Connection, id: i64) -> Result<Option<Project>, Error>{
        let mut stmt = db.prepare("SELECT id, name, strftime('%s', created_at) as created_at from projects where id = :id")?;
        stmt.bind::<&[(_, Value)]>(&[
            (":id", id.into()),
        ])?;
        let mut project = None;
        while let Ok(State::Row) = stmt.next() {
            project = Some(Project{
                id: Some(stmt.read::<i64, _>("id").unwrap()),
                name: stmt.read::<String, _>("name").unwrap(),
                created_at: DateTime::from_timestamp(stmt.read::<i64, _>("created_at").unwrap(), 0).map(|t|t.with_timezone(&Local))
            });
            break;
        }

        Ok(project)
    }*/

    pub fn find_by_name(db: &Connection, name: String) -> Result<Option<Project>, Error>{
        let mut stmt = db.prepare("SELECT id, name, strftime('%s', created_at) as created_at from projects where name = :name")?;
        stmt.bind::<&[(_, Value)]>(&[
            (":name", name.into()),
        ])?;
        let mut project = None;
        while let Ok(State::Row) = stmt.next() {
            project = Some(Project::from(&mut stmt));
            break;
        }

        Ok(project)
    }

    pub fn list(db: &Connection) -> Result<Vec<Project>, Error> {
        let mut stmt = db.prepare("SELECT id, name, strftime('%s', created_at) as created_at from projects")?;
        let mut projects = vec![];
        while let Ok(State::Row) = stmt.next() {
            projects.push(Project::from(&mut stmt));
        }
        Ok(projects)
    }

    pub fn insert(db: &Connection, project: Project) -> Result<sqlite::Result<State>, Error> {
        let mut stmt = db.prepare("INSERT INTO projects (id, name) VALUES (null, :name)").unwrap();
        stmt.bind::<&[(_, Value)]>(&[
            (":name", project.name.clone().into()),
        ])?;

        Ok(stmt.next())
    }
    pub fn delete(db: &Connection, task_id: i64) -> Result<sqlite::Result<State>, Error> {
        let mut delete_tasks = db.prepare("DELETE from tasks where project_id = :id").unwrap();
        delete_tasks.bind::<&[(_, Value)]>(&[
            (":id", task_id.into()),
        ])?;
        delete_tasks.next()?;

        let mut stmt = db.prepare("DELETE from projects where id = :id").unwrap();
        stmt.bind::<&[(_, Value)]>(&[
            (":id", task_id.into()),
        ])?;

        Ok(stmt.next())
    }

    pub fn sum_work_time(db: &Connection, project_id: i64, from: DateTime<Local>) -> Result<i64, Error> {
        let mut stmt = db.prepare("SELECT Sum(time) as sum_time from tasks where project_id = :project_id and strftime('%s', created_at) >= :from")?;

        stmt.bind::<&[(_, Value)]>(&[
            (":project_id", project_id.into()),
            (":from", from.timestamp().into()),
        ])?;

        let sum = 0;
        while let Ok(State::Row) = stmt.next() {
            return Ok(stmt.read::<i64, _>("sum_time").unwrap())
        }
        Ok(sum)
    }
}