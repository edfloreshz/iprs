#[cfg(not(windows))]
use signal_hook::{iterator::Signals, SIGINT};
#[cfg(not(windows))]
use std::process;
#[cfg(not(windows))]
use std::thread;

use dirs::{desktop_dir, document_dir, download_dir};
use notify::{op, raw_watcher, RawEvent, RecursiveMode, Watcher};

use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver, Sender};

use crate::errors::custom::CustomError;
use crate::replication::engine::*;
use crate::Result;

#[cfg(not(windows))]
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

#[cfg(not(windows))]
pub fn init() -> Result<()> {
    println!("Initializing the daemon...");
    // Create a channel to receive the events.
    let (tx, rx) = channel();
    let directories: Vec<Option<PathBuf>> = vec![download_dir(), document_dir(), desktop_dir()];
    manage_signals()?;
    watch_events(directories, rx, tx)
}

fn watch_events(
    directories: Vec<Option<PathBuf>>,
    rx: Receiver<RawEvent>,
    tx: Sender<RawEvent>,
) -> Result<()> {
    let mut watcher = raw_watcher(tx).unwrap();
    for directory in directories {
        match directory {
            Some(dir) => Ok(watcher.watch(dir, RecursiveMode::Recursive).unwrap()),
            None => Err(CustomError::new("Cannot find directory")),
        }?;
    }
    println!("Waiting for changes...");
    loop {
        // TODO: Retrieve the ID from the database in order to keep track of the files.
        match rx.recv() {
            Ok(RawEvent {
                path: Some(path),
                op: Ok(op),
                cookie,
            }) => {
                println!("{:?} {:?} ({:?})", op, path, cookie);
                match op {
                    op::CREATE => add(vec![path.clone()])?,
                    op::WRITE => (),
                    op::RENAME => rename(vec![path.clone()])?,
                    op::REMOVE => {
                        remove(vec![path.clone().into_os_string().into_string().unwrap()])?
                    }
                    op::CHMOD => update(vec![path.clone()])?,
                    op::CLOSE_WRITE => update(vec![path.clone()])?,
                    op::RESCAN => (),
                    _ => println!("Unhandled event"),
                };
            }
            Ok(event) => println!("broken event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

#[cfg(windows)]
pub fn init() -> Result<()> {
    println!("Initializing the daemon...");
    // Create a channel to receive the events.
    let (tx, rx) = channel();
    let directories: Vec<Option<PathBuf>> = vec![download_dir(), document_dir(), desktop_dir()];
    watch_events(directories, rx, tx)
}
