use os_info;
use crate::utils::os;
use std::thread;
use std::time::Duration;

#[cfg(target_os = "macos")]
fn run()  -> Result<(), String> {
  println!("Installing IPSS for {}", os_info::get());
  let mut bar = progress::Bar::new();
  bar.set_job_title(format!("Installing IPSS for {}", os::print_os_info()).as_str());
  for i in 0..11 {
    thread::sleep(Duration::from_millis(100));
    bar.reach_percent(i * 10);
  }
  println!("\nIPSS is now installed.")
}

#[cfg(target_os = "windows")]
fn run()  -> Result<(), String> {
  println!("Installing IPSS for {}", os_info::get());
  let mut bar = progress::Bar::new();
  bar.set_job_title(format!("Installing IPSS for {}", os::print_os_info()).as_str());
  for i in 0..11 {
    thread::sleep(Duration::from_millis(100));
    bar.reach_percent(i * 10);
  }
  println!("\nIPSS is now installed.")
}

#[cfg(target_os = "linux")]
fn run() -> Result<(), String> {
  let mut bar = progress::Bar::new();
  bar.set_job_title(format!("Installing IPSS for {}", os::print_os_info()).as_str());
  for i in 0..11 {
    thread::sleep(Duration::from_millis(100));
    bar.reach_percent(i * 10);
  }
  println!("\nIPSS is now installed.");
  Ok(())
}

pub fn install() -> Result<(), String> {
  match os_info::get().os_type() {
    os_info::Type::Windows => run(),
    os_info::Type::Macos => run(),
    _ => run()
  }
}

