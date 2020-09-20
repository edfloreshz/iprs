pub mod utils;
pub mod ipss;
pub mod replication;
pub mod errors;
use crate::errors::custom::CustomError;

pub enum InstallStatus {
  Installed(String),
  Error(Box<CustomError>)
}