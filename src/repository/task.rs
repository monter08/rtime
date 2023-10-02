use sqlite::{Connection, Error, State, Value};
use crate::model::{Project, Task};

pub struct TaskRepository {
    pub db: Connection,
}

impl TaskRepository {
    pub fn init(db: &Connection) {
        db.execute("CREATE TABLE IF NOT EXISTS tasks (id	INTEGER, project_id	INTEGER NOT NULL, name	TEXT, time	INTEGER, PRIMARY KEY(id AUTOINCREMENT), FOREIGN KEY(project_id) REFERENCES projects(id) ON DELETE CASCADE)")
            .expect("Cannot create table tasks");
    }
    pub fn list(db: &Connection) -> Result<Vec<Task>, Error> {
        let mut stmt = db.prepare("SELECT tasks.id as task_id, tasks.name as task_name, tasks.time as task_time, projects.id as project_id, projects.name as project_name from tasks left join projects on tasks.project_id = projects.id")?;
        let mut tasks = vec![];
        while let Ok(State::Row) = stmt.next() {
            tasks.push(Task{
                id: Some(stmt.read::<i64, _>("task_id").unwrap()),
                project: Project {
                    id: Some(stmt.read::<i64, _>("project_id").unwrap()),
                    name: stmt.read::<String, _>("project_name").unwrap()
                },
                name: stmt.read::<String, _>("task_name").unwrap(),
                time: stmt.read::<i64, _>("task_time").unwrap(),
            });
        }
        Ok(tasks)
    }

    pub fn find(db: &Connection, project_id: i64) -> Result<Vec<Task>, Error> {
        let mut stmt = db.prepare("SELECT tasks.id as task_id, tasks.name as task_name, tasks.time as task_time, projects.id as project_id, projects.name as project_name from tasks left join projects on tasks.project_id = projects.id WHERE projects.id = :project_id").unwrap();
        stmt.bind::<&[(_, Value)]>(&[
            (":project_id", project_id.into()),
        ]).expect("TODO: panic message");
        let mut tasks = vec![];
        while let Ok(State::Row) = stmt.next() {
            tasks.push(Task{
                id: Some(stmt.read::<i64, _>("task_id").unwrap()),
                project: Project {
                    id: Some(stmt.read::<i64, _>("project_id").unwrap()),
                    name: stmt.read::<String, _>("project_name").unwrap()
                },
                name: stmt.read::<String, _>("task_name").unwrap(),
                time: stmt.read::<i64, _>("task_time").unwrap(),
            });
        }
        Ok(tasks)
    }


    pub fn insert(db: &Connection, task: Task) -> sqlite::Result<State> {
        let mut stmt = db.prepare("INSERT INTO tasks (id, project_id, name, time) VALUES (null, :project_id, :name, :time)").unwrap();
        stmt.bind::<&[(_, Value)]>(&[
            (":project_id", task.project.id.unwrap().into()),
            (":name", task.name.clone().into()),
            (":time", task.time.clone().into()),
        ]).expect("TODO: panic message");


        stmt.next()
    }

    pub fn delete(db: &Connection, task_id: i64) -> sqlite::Result<State> {
        let mut stmt = db.prepare("DELETE from tasks where id = :id").unwrap();
        stmt.bind::<&[(_, Value)]>(&[
            (":id", task_id.into()),
        ]).expect("TODO: panic message");

        stmt.next()
    }
}
