use crate::{Action, QueueState, Result};
use nanoid::nanoid;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::fmt;

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

pub fn add(input: Vec<PathBuf>) -> Result<()> {
    for file in input.iter() {
        if file.exists() {
            let mut queued_file = QueuedFile::new(file.to_path_buf(), Action::Modify);
            queued_file.upload()?
        }
    }
    Ok(())
}

pub fn rename(input: Vec<PathBuf>) -> Result<()> {
    for file in input.iter() {
        if file.exists() {
            println!("{}", file.as_path().display());
            let mut queued_file = QueuedFile::new(file.to_path_buf(), Action::Modify);
            queued_file.upload()?
        }
    }
    Ok(())
}

pub fn update(input: Vec<PathBuf>) -> Result<()> {
    for file in input.iter() {
        println!("Uploading {}", file.display());
        if file.exists() {
            let mut queued_file = QueuedFile::new(file.to_path_buf(), Action::Modify);
            queued_file.upload()?
        }
    }
    Ok(())
}

pub fn get(input: Vec<String>) -> Result<()> {
    for file in input.iter() {
        println!("Getting {}", file)
    }
    Ok(())
}

pub fn remove(input: Vec<String>) -> Result<()> {
    for file in input.iter() {
        println!("Removing {}", file)
    }
    Ok(())
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
