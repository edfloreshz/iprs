use os_info;

pub fn print_os_info() -> String {
  format!("{}", os_info::get())
}