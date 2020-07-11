use notify::{Watcher, RecursiveMode, RawEvent, raw_watcher};
use std::sync::mpsc::channel;
use dirs;

pub fn init() {
  println!("Initializing the daemon...");
  // Create a channel to receive the events.
  let (tx, rx) = channel();

  // Create a watcher object, delivering raw events.
  // The notification back-end is selected based on the platform.
  let mut watcher = raw_watcher(tx).unwrap();

  // Add a path to be watched. All files and directories at that path and
  // below will be monitored for changes.
  if let Some(dir) = dirs::download_dir() {
    watcher.watch(dir, RecursiveMode::Recursive).unwrap();
  }

  println!("Waiting for changes...");
  loop {
    match rx.recv() {
      Ok(RawEvent{path: Some(path), op: Ok(op), cookie}) => {
        println!("{:?} {:?} ({:?})", op, path, cookie)
      },
      Ok(event) => println!("broken event: {:?}", event),
      Err(e) => println!("watch error: {:?}", e),
    }
  }
}