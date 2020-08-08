use std::fs::File;
use crate::replication::node::find_nodes;
use std::error;
use std::path::Path;

pub fn add(path: &Path) -> Result<(), Box<dyn error::Error>> {
  let file = File::open(path);
  match file {
    Ok(file) => Ok(queue(QueuedFile::new(&path, Action::Create), &file)),
    Err(e) => Err(Box::new(e))
  }
}

pub fn rename(_path: &Path) -> Result<(), Box<dyn error::Error>> {
  // Ok(queue(QueuedFile::new(path, Action::Rename)))
  Ok(())
}

pub fn modify(_path: &Path) -> Result<(), Box<dyn error::Error>> {
  Ok(())
}

pub fn cat() {}

pub fn get() {}

pub fn remove(_path: &Path) -> Result<(), Box<dyn error::Error>> {
  // Ok(queue(QueuedFile::new(path, Action::Remove)))
  Ok(())
}

// Private API
#[allow(dead_code)]
struct QueuedFile<'a> {
  name: String,
  path: &'a Path,
  tracking_id: String,
  action: Action,
  state: QueueState
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
  pub fn upload(&mut self, _file: &File) -> String { // TODO: Upload file to IPFS and return the key
    let key = "".to_string();
    self.state = QueueState::Uploaded;
    key
  }

}

pub fn generate_tracking_id() -> String { // TODO: Generate tracking ID for files.
  String::new()
}

fn queue(mut queued_file: QueuedFile, file: &File) {
  println!("File will be sent to queue... [Yet to implement]");
  match queued_file.action {
    Action::Create => {
      find_nodes(queued_file.upload(file))
    }
    Action::Modify => {}
    Action::Rename => {}
    Action::Remove => {}
    Action::Nothing => {}
  }
}