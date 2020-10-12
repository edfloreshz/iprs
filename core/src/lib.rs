use std::fmt;

pub mod errors;
pub mod ipss;
pub mod replication;

use std::error;
type Result<T> = std::result::Result<T, Box<dyn error::Error + 'static>>;

#[derive(Clone)]
pub enum Action {
    Create,
    Modify,
    Rename,
    Remove,
    Nothing,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Action::Create => write!(f, "Create"),
            Action::Modify => write!(f, "Modify"),
            Action::Rename => write!(f, "Rename"),
            Action::Remove => write!(f, "Remove"),
            Action::Nothing => write!(f, "Nothing"),
        }
    }
}

#[derive(Clone)]
pub enum QueueState {
    Local,
    Failed,
    Uploaded,
}

impl fmt::Display for QueueState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QueueState::Local => write!(f, "Local"),
            QueueState::Uploaded => write!(f, "Uploaded"),
            QueueState::Failed => write!(f, "Failed"),
        }
    }
}
