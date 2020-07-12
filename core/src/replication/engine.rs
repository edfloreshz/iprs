

// Public API

use std::fs::File;

pub fn add(file: File) {
  queue(QueuedFile::new(file));
}

pub fn cat() {}

pub fn get() {}

pub fn remove() {}

// Private API

struct QueuedFile {
    name: String,
    directory: String,
    tracking_id: String,
    state: QueueState
}

enum QueueState {
    Local,
    Uploading,
    Uploaded,
}

impl QueuedFile { // TODO: Write QueuedFile constructor
    pub fn new(_file: File) -> QueuedFile {
        QueuedFile {
            name: "".to_string(),
            directory: "".to_string(),
            tracking_id: "".to_string(),
            state: QueueState::Local
        }
    }
    pub fn upload(&mut self) {
        self.state = QueueState::Uploading
    }
    pub fn uploaded(&mut self) {
        self.state = QueueState::Uploaded
    }
}

fn queue(mut file: QueuedFile) {
    println!("File will be sent to queue... [Yet to implement]");
    file.upload()
}