// use std::fs::File;
use crate::Result;
use dirs;
use notify::{op, raw_watcher, RawEvent, RecursiveMode, Watcher};
#[cfg(not(target_os = "windows"))]
use std::process;
use std::sync::mpsc::{channel, Receiver, Sender};
#[cfg(not(target_os = "windows"))]
use std::thread;

use crate::errors::custom::CustomError;
use crate::replication::engine::QueuedFile;
use crate::Action;
use queue::Queue;
#[cfg(not(target_os = "windows"))]
use signal_hook::{iterator::Signals, SIGINT};
use std::path::PathBuf;

#[cfg(not(target_os = "windows"))]
pub fn init() -> Result<()> {
    println!("Initializing the daemon...");
    // Create a channel to receive the events.
    let (tx, rx) = channel();
    let directories: Vec<Option<PathBuf>> = vec![
        dirs::download_dir(),
        dirs::document_dir(),
        dirs::desktop_dir(),
    ];
    add_watch_directories(directories, tx)?;
    manage_signals()?;
    watch_events(rx)
}

fn add_watch_directories(directories: Vec<Option<PathBuf>>, tx: Sender<RawEvent>) -> Result<()> {
    // Create a watcher object, delivering raw events.
    // The notification back-end is selected based on the platform.
    let mut watcher = raw_watcher(tx).unwrap();
    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    for directory in directories {
        match directory {
            Some(dir) => Ok(watcher.watch(dir, RecursiveMode::Recursive).unwrap()),
            None => Err(CustomError::new("Cannot find directory")),
        }?;
    }
    Ok(())
}

fn manage_signals() -> Result<()> {
    let signals = Signals::new(&[SIGINT])?;
    thread::spawn(move || {
        for sig in signals.forever() {
            if let 2 = sig {
                println!("\nExiting the daemon...");
                process::exit(0)
            }
        }
    });
    Ok(())
}

fn watch_events(rx: Receiver<RawEvent>) -> Result<()> {
    let mut file_queue: Queue<QueuedFile> = Queue::new();
    println!("Waiting for changes...");
    loop {
        // TODO: The tracking ID is generated for each event, need to retrieve the ID from the database in order to keep track of the files.
        match rx.recv() {
            Ok(RawEvent {
                path: Some(path),
                op: Ok(op),
                cookie,
            }) => {
                println!("{:?} {:?} ({:?})", op, path, cookie);
                match op {
                    op::CREATE => {
                        if let Ok(..) =
                            file_queue.queue(QueuedFile::new(path.clone(), Action::Create))
                        {
                            if let Some(item) = file_queue.dequeue() {
                                println!("{}", item)
                            }
                        }
                    }
                    op::CLOSE_WRITE => {
                        if let Ok(..) =
                            file_queue.queue(QueuedFile::new(path.clone(), Action::Modify))
                        {
                            if let Some(item) = file_queue.dequeue() {
                                println!("{}", item)
                            }
                        }
                    }
                    op::REMOVE => {
                        if let Ok(..) =
                            file_queue.queue(QueuedFile::new(path.clone(), Action::Remove))
                        {
                            if let Some(item) = file_queue.dequeue() {
                                println!("{}", item)
                            }
                        }
                    }
                    op::RENAME => {
                        if let Ok(..) =
                            file_queue.queue(QueuedFile::new(path.clone(), Action::Rename))
                        {
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

    let mut file_queue: Queue<QueuedFile> = Queue::new();
    println!("Waiting for changes...");
    loop {
        // TODO: The tracking ID is generated for each event, need to retrieve the ID from the database in order to keep track of the files.
        match rx.recv() {
            Ok(RawEvent {
                path: Some(path),
                op: Ok(op),
                cookie,
            }) => {
                println!("{:?} {:?} ({:?})", op, path, cookie);
                match op {
                    op::CREATE => {
                        if let Ok(..) =
                            file_queue.queue(QueuedFile::new(path.clone(), Action::Create))
                        {
                            if let Some(item) = file_queue.dequeue() {
                                println!("{}", item)
                            }
                        }
                    }
                    op::CLOSE_WRITE => {
                        if let Ok(..) =
                            file_queue.queue(QueuedFile::new(path.clone(), Action::Modify))
                        {
                            if let Some(item) = file_queue.dequeue() {
                                println!("{}", item)
                            }
                        }
                    }
                    op::REMOVE => {
                        if let Ok(..) =
                            file_queue.queue(QueuedFile::new(path.clone(), Action::Remove))
                        {
                            if let Some(item) = file_queue.dequeue() {
                                println!("{}", item)
                            }
                        }
                    }
                    op::RENAME => {
                        if let Ok(..) =
                            file_queue.queue(QueuedFile::new(path.clone(), Action::Rename))
                        {
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
