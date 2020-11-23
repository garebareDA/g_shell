use std::env;

use dirs;

pub fn run_cd(argvs: &Vec<&str>) -> Result<(), String> {
  if argvs[0] == "cd" {
    if argvs.len() != 2 {
      env::set_current_dir(dirs::home_dir().unwrap()).unwrap();
    } else if !env::set_current_dir(argvs[1]).is_ok() {
      return Err(format!("cd {}: No such file or directory\n", argvs[1]));
    }
  }
  return Ok(());
}
