// use std::fs::File;
use dirs;
use notify::{op, raw_watcher, RawEvent, RecursiveMode, Watcher};
use std::error::Error;
use std::sync::mpsc::channel;

#[cfg(not(target_os = "windows"))]
use std::process;
#[cfg(not(target_os = "windows"))]
use std::thread;

use crate::replication::engine::QueuedFile;
use crate::Action;
use queue::Queue;
#[cfg(not(target_os = "windows"))]
use signal_hook::{iterator::Signals, SIGINT};

#[cfg(not(target_os = "windows"))]
pub fn init() -> Result<(), Box<dyn Error>> {
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
    if let Some(dir) = dirs::document_dir() {
        watcher.watch(dir, RecursiveMode::Recursive).unwrap();
    }
    if let Some(dir) = dirs::desktop_dir() {
        watcher.watch(dir, RecursiveMode::Recursive).unwrap();
    }
    let signals = Signals::new(&[SIGINT])?;
    thread::spawn(move || {
        for sig in signals.forever() {
            match sig {
                2 => {
                    println!("\nExiting the daemon...");
                    process::exit(0)
                }
                _ => {}
            }
        }
    });

    let mut file_queue: Queue<QueuedFile> = Queue::new();
    println!("Waiting for changes...");
    loop { // TODO: The tracking ID is generated for each event, need to retrieve the ID from the database in order to keep track of the files.
        match rx.recv() {
            Ok(RawEvent {
                path: Some(path),
                op: Ok(op),
                cookie,
            }) => {
                println!("{:?} {:?} ({:?})", op, path, cookie);
                match op {
                    op::CREATE => {
                        if let Ok(..) = file_queue.queue(QueuedFile::new(path.clone(),
                                                                         Action::Create)) {
                            if let Some(item) = file_queue.dequeue() {
                                println!("{}", item)
                            }
                        }
                    }
                    op::CLOSE_WRITE => {
                        if let Ok(..) = file_queue.queue(QueuedFile::new(path.clone(),
                                                                         Action::Modify)) {
                            if let Some(item) = file_queue.dequeue() {
                                println!("{}", item)
                            }
                        }
                    }
                    op::REMOVE => {
                        if let Ok(..) = file_queue.queue(QueuedFile::new(path.clone(),
                                                                         Action::Remove)) {
                            if let Some(item) = file_queue.dequeue() {
                                println!("{}", item)
                            }
                        }
                    }
                    op::RENAME => {
                        if let Ok(..) = file_queue.queue(QueuedFile::new(path.clone(),
                                                                         Action::Rename)) {
                            if let Some(item) = file_queue.dequeue() {
                                println!("{}", item)
                            }
                        }
                    }
                    _ => println!("Unhandled event"),
                };
            }
            Ok(event) => println!("broken event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

#[cfg(target_os = "windows")]
pub fn init() -> Result<(), Box<dyn Error>> {
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
    if let Some(dir) = dirs::document_dir() {
        watcher.watch(dir, RecursiveMode::Recursive).unwrap();
    }
    if let Some(dir) = dirs::desktop_dir() {
        watcher.watch(dir, RecursiveMode::Recursive).unwrap();
    }

    println!("Waiting for changes...");
    loop {
        match rx.recv() {
            Ok(RawEvent {
                path: Some(path),
                op: Ok(op),
                cookie,
            }) => {
                println!("{:?} {:?} ({:?})", op, path, cookie);
                let _ = match op {
                    op::CREATE => engine::add(&path),
                    op::CLOSE_WRITE => engine::modify(&path),
                    op::REMOVE => engine::remove(&path),
                    op::RENAME => engine::rename(&path),
                    _ => Ok(()),
                };
            }
            Ok(event) => println!("broken event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
