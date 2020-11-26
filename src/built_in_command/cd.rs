use std::env;

use dirs;

use super::super::parser::parser::CommandParse;

pub fn run_cd(commands: &CommandParse) -> Result<(), String> {
  if commands.command == "cd" {
    if commands.path.trim().is_empty() {
      env::set_current_dir(dirs::home_dir().unwrap()).unwrap();
    } else if !env::set_current_dir(&commands.path).is_ok() {
      return Err(format!("cd {}: No such file or directory\n", commands.path));
    }
  }
  return Ok(());
}
