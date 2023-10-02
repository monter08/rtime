use sqlite::Connection;
struct ProjectRepository {
    db: Connection,
}

impl ProjectRepository {
    fn list(self) -> Vec<Task> {

    }
}