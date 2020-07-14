use std::fs::File;
use crate::replication::node::find_nodes;

pub fn add(file: &File) {
  queue(QueuedFile::new(file, Action::Create));
}

pub fn rename(file: &File) {
    queue(QueuedFile::new(file, Action::Rename))
}

pub fn modify(_file: &File) {}

pub fn cat() {}

pub fn get() {}

pub fn remove(file: &File) {
    queue(QueuedFile::new(file, Action::Remove))
}

// Private API
#[allow(dead_code)]
struct QueuedFile {
    name: String,
    path: String,
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

impl QueuedFile { // TODO: Write QueuedFile constructor
    pub fn new(_file: &File, action: Action) -> QueuedFile {
        QueuedFile {
            name: "".to_string(),
            path: "".to_string(),
            tracking_id: "".to_string(),
            action,
            state: QueueState::Local
        }
    }
    pub fn upload(&mut self) -> String { // TODO: Upload file to IPFS and return the key
        let key = "".to_string();
        self.state = QueueState::Uploaded;
        key
    }
}

fn queue(mut file: QueuedFile) {
    println!("File will be sent to queue... [Yet to implement]");
    match file.action {
        Action::Create => {
            find_nodes(file.upload())
        }
        Action::Modify => {}
        Action::Rename => {}
        Action::Remove => {}
        Action::Nothing => {}
    }
}