
use clap::{Parser, Subcommand};
use dialoguer::Select;
use sqlite::Connection;
use crate::cli::{git, project, task as tasks};
use crate::model::Project;
use crate::repository::ProjectRepository;

#[derive(Subcommand, Debug)]
enum Commands {
    /// Manage tasks
    #[clap(subcommand, alias="t")]
    Task(TaskCommand),
    /// Manage projects
    #[clap(subcommand, alias="p")]
    Project(ProjectCommand),
    /// Add Git hook to repository
    #[clap(subcommand, alias="g")]
    Git(GitCommand)
}

#[derive(Subcommand, Debug)]
pub enum TaskCommand {
    #[clap(alias="l")]
    List {
        project: Option<i64>,
    },
    #[clap(alias="c")]
    Create {
        #[arg(short, long)]
        name: Option<String>,
        project: Option<String>,
        time: Option<String>,
    },
    #[clap(alias="d")]
    Delete {
        task_id: Option<i64>
    },
}

#[derive(Subcommand, Debug)]
pub enum ProjectCommand {
    #[clap(alias="l")]
    List,
    #[clap(alias="c")]
    Create {
        name: Option<String>,
    },
    #[clap(alias="d")]
    Delete {
        name: Option<String>
    },
}

#[derive(Subcommand, Debug)]
pub enum GitCommand {
    #[clap(alias="i")]
    Init {
        project: Option<String>,
    },
    #[clap(alias="c")]
    Clean,
}


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Clone)]
pub struct Error {
    details: String
}

impl Error {
    pub fn new(details: String) -> Self {
        Error {
            details,
        }
    }
}

impl ToString for Error {
    fn to_string(&self) -> String {
        self.details.to_string()
    }
}

impl From<sqlite::Error> for Error {
    fn from(value: sqlite::Error) -> Self {
        Self {
            details: format!("DB error: {}", value.to_string())
        }
    }
}


pub fn start(db: &Connection) -> Result<(), Error> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Task(task) => {
            match task {
                TaskCommand::List {project} => tasks::list(db, project),
                TaskCommand::Create { name, project, time } => tasks::create(db, name, project, time),
                TaskCommand::Delete { task_id } => tasks::delete(db, task_id)
            }
        }
        Commands::Project(command) => {
            match command {
                ProjectCommand::List => project::list(db),
                ProjectCommand::Create { name} => project::create(db, name),
                ProjectCommand::Delete { name } => project::delete(db, name)
            }
        }
        Commands::Git(command) => {
            match command {
                GitCommand::Init {project} => git::init(db, project),
                GitCommand::Clean => git::clean(),
            }
        }
    }
}

pub fn select_project(db: &Connection, name: Option<String>) -> Result<Project, Error> {
    if name.is_some() {
        return match ProjectRepository::find_by_name(db, name.clone().unwrap())? {
            None => {
                Err(Error::new(format!("Project: {} does not exists", name.clone().unwrap())))
            }
            Some(p) => {
                Ok(p)
            }
        }
    }

    let list = ProjectRepository::list(&db)?;

    if list.is_empty() {
        return Err(Error::new("Project list is empty, add project first".to_string())) // format!("{} {}", "Add project first, try:".red(), "rtime project new".red().bold()
    }
    else {
        let selection = Select::new()
            .with_prompt("Select project")
            .default(0)
            .items(&list)
            .interact()
            .unwrap();

        Ok(ProjectRepository::find_by_name(db, list[selection].to_string())?.unwrap())
    }
}

pub fn minutes_display(m: i64) -> String {
    format!("{}h {}m", m / 60, m % 60)
}
