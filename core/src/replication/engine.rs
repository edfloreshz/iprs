use crate::{Action, QueueState, Result};
use nanoid::nanoid;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Clone)]
pub struct QueuedFile {
    name: String,
    path: PathBuf,
    tracking_id: String,
    action: Action,
    state: QueueState,
}

impl QueuedFile {
    pub fn new(path: PathBuf, action: Action) -> QueuedFile {
        let name = match path.file_name() {
            Some(filename) => filename.to_string_lossy().to_string(),
            None => panic!("Couldn't get filename!"),
        };
        QueuedFile {
            name,
            path,
            tracking_id: generate_tracking_id(),
            action,
            state: QueueState::Local,
        }
    }
    pub fn upload(&mut self) -> Result<()> {
        File::open(&self.path)?;
        match self.replicate() {
            Ok(..) => {
                self.state = QueueState::Uploaded;
                println!("Uploaded file: {}", self);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
    fn replicate(&self) -> Result<()> {
        // TODO: Mark file for replication
        Ok(())
    }
}

impl fmt::Display for QueuedFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\n    name: {}, \n    path: {}, \n    id: {}, \n    action: {}, \n    state: {}\n",
            self.name,
            self.path.to_str().unwrap(),
            self.tracking_id,
            self.action,
            self.state
        )
    }
}

pub fn add(input: PathBuf) -> Result<()> {
    if input.exists() {
        let mut queued_file = QueuedFile::new(input.to_path_buf(), Action::Modify);
        queued_file.upload()?
    }
    Ok(())
}

pub fn rename(input: PathBuf) -> Result<()> {
    if input.exists() {
        println!("{}", input.as_path().display());
        let mut queued_file = QueuedFile::new(input.to_path_buf(), Action::Modify);
        queued_file.upload()?
    }
    Ok(())
}

pub fn update(input: PathBuf) -> Result<()> {
    println!("Uploading {}", input.display());
    if input.exists() {
        let mut queued_file = QueuedFile::new(input.to_path_buf(), Action::Modify);
        queued_file.upload()?
    }
    Ok(())
}

pub fn get(input: PathBuf) -> Result<()> {
    Ok(println!("Getting {}", input.display()))
}

pub fn remove(input: PathBuf) -> Result<()> {
    Ok(println!("Removing {}", input.display()))
}

pub fn cat(path: PathBuf) -> Result<()> {
    match File::open(path) {
        Ok(file) => Ok(for line in BufReader::new(file).lines() {
            println!("{}", line.unwrap())
        }),
        Err(e) => Err(Box::new(e)),
    }
}

fn generate_tracking_id() -> String {
    nanoid!(10)
}
