use std::error;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;

#[allow(dead_code)]
struct QueuedFile<'a> {
  name: String,
  path: &'a Path,
  tracking_id: String,
  action: Action,
  state: QueueState
}

impl QueuedFile<'_> {
  pub fn new(path: &Path, action: Action) -> QueuedFile {
    let name= match path.file_name() {
      Some(filename) => filename.to_string_lossy().to_string(),
      None => panic!("Couldn't get filename!")
    };
    QueuedFile {
      name,
      path,
      tracking_id: generate_tracking_id(),
      action,
      state: QueueState::Local
    }
  }
  pub fn upload(&mut self) -> Result<(), Box<dyn error::Error>> {
    let file = File::open(self.path);
    match file {
      Ok(_file) => {
        //TODO: Upload file
        self.state = QueueState::Uploaded;
        Ok(())
      } ,
      Err(e) => Err(Box::new(e))
    }
  }
}
#[allow(dead_code)]
enum Action {
  Create,
  Modify,
  Rename,
  Remove,
  Nothing,
}
#[allow(dead_code)]
enum QueueState {
  Local,
  Failed,
  Uploaded,
}

fn queue(mut queued_file: QueuedFile) -> Result<(), Box<dyn error::Error>> {
  println!("File will be sent to queue... [Yet to implement]");
  match queued_file.action {
    Action::Create => queued_file.upload(),
    Action::Modify => Ok(()),
    Action::Rename => Ok(()),
    Action::Remove => Ok(()),
    Action::Nothing => Ok(()),
  }
}

pub fn add(path: &Path) -> Result<(), Box<dyn error::Error>> {
  queue(QueuedFile::new(&path, Action::Create))
}

pub fn rename(path: &Path) -> Result<(), Box<dyn error::Error>> {
  queue(QueuedFile::new(path, Action::Rename))
}

pub fn modify(_path: &Path) -> Result<(), Box<dyn error::Error>> {
  Ok(())
}

pub fn cat(path: &Path) -> Result<(), Box<dyn error::Error>> {
  match File::open(path) {
    Ok(file) => {
      Ok(for line in BufReader::new(file).lines() {
        println!("{}", line.unwrap())
      })
    },
    Err(e) => Err(Box::new(e))
  }
}

pub fn get() {}

pub fn remove(_path: &Path) -> Result<(), Box<dyn error::Error>> {
  // Ok(queue(QueuedFile::new(path, Action::Remove)))
  Ok(())
}

pub fn generate_tracking_id() -> String { // TODO: Generate tracking ID for files.
  String::new()
}