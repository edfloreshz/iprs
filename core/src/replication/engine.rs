use crate::{Action, QueueState};
use nanoid::nanoid;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::{env, error, fmt};

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
    pub fn upload(&mut self) -> Result<(), Box<dyn error::Error>> {
        let file = File::open(&self.path);
        match file {
            Ok(..) => {
                //TODO: Upload file
                self.state = QueueState::Uploaded;
                println!("{}", self);
                Ok(())
            }
            Err(e) => Err(Box::new(e)),
        }
    }
}

impl fmt::Display for QueuedFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Queue file: ( \n    name: {}, \n    path: {}, \n    id: {}, \n    action: {}, \n    state: {}\n)",
            self.name,
            self.path.to_str().unwrap(),
            self.tracking_id,
            self.action,
            self.state
        )
    }
}

pub fn add(input: Vec<String>) -> Result<(), Box<dyn error::Error>> {
    for file in input.iter() {
        let path = Path::new(&env::current_dir()?).join(file);
        println!("Uploading {}", path.display());
        let mut queued_file = QueuedFile::new(path, Action::Create);
        queued_file.upload()?
    }
    Ok(())
}

pub fn get(input: Vec<String>) -> Result<(), Box<dyn error::Error>> {
    for file in input.iter() {
        println!("Getting {}", file)
    }
    Ok(())
}

pub fn remove(input: Vec<String>) -> Result<(), Box<dyn error::Error>> {
    for file in input.iter() {
        println!("Removing {}", file)
    }
    Ok(())
}

pub fn cat(path: PathBuf) -> Result<(), Box<dyn error::Error>> {
    match File::open(path) {
        Ok(file) => Ok(for line in BufReader::new(file).lines() {
            println!("{}", line.unwrap())
        }),
        Err(e) => Err(Box::new(e)),
    }
}

pub fn generate_tracking_id() -> String {
    nanoid!(10)
}
