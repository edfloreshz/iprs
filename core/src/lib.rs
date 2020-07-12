pub mod utils;
pub mod ipss;
pub mod replication;

pub enum InstallStatus {
  Installed,
  Error(String)
}