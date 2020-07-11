pub mod utils;
pub mod ipss;

pub enum InstallStatus {
  Installed,
  Error(String)
}