// use std::fs::File;
//
// pub struct QueuedFile {
//     name: String,
//     directory: String,
//     tracking_id: String,
//     state: QueueState
// }
//
// impl QueueState {
//     pub fn new(&self, _file: File) -> QueuedFile {
//         QueuedFile {
//             name: "".to_string(),
//             directory: "".to_string(),
//             tracking_id: "".to_string(),
//             state: QueueState::Local
//         }
//     }
// }
//
// enum QueueState {
//     Local,
//     Uploading,
//     Uploaded,
// }
//
// pub fn queue() {}