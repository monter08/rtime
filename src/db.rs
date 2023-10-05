use std::ffi::OsString;
use std::path::PathBuf;
use sqlite::Connection;
use std::fs;

#[derive(Clone, Debug)]
pub struct DB {
    pub filename: String,
}

impl DB {
    pub fn connect(self) -> Connection{
        sqlite::open(&self.filename).unwrap_or_else(|err| panic!("Failed to open file: {}, error: {}", &self.filename, err))
    }

    pub fn path() -> Result<String, OsString> {
        let mut path = match home::home_dir() {
            Some(path) => path,
            None => PathBuf::new(),
        };
        path.push(".rtime");

        if !path.exists() {
            fs::create_dir(path.clone()).expect("Cannot create directory :(")
        }

        path.push("db.sqlite");
        path.into_os_string().into_string()
    }

}