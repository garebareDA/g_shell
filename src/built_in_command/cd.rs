use std::env;

use dirs;

use super::super::parser::parser::CommandParse;

pub fn run_cd(commands: &CommandParse) -> Result<(), String> {
  if commands.get_command() == "cd" {
    let is_path = set_current(&commands.get_path());
    let is_subcommand = set_current(&commands.get_sub_command());
    let is_path_empty = commands.get_path().trim().is_empty();
    let is_subcommand_empty = commands.get_sub_command().trim().is_empty();

    if is_path_empty && is_subcommand_empty {
      env::set_current_dir(dirs::home_dir().unwrap()).unwrap();
      return Ok(());
    }

    if !(is_path || is_subcommand) {
      if !is_path_empty {
        return Err(format!("cd : {} No such file or directory", &commands.get_path()));
      } else if !is_subcommand_empty {
        return Err(format!(
          "cd : {} No such file or directory",
          &commands.get_sub_command()
        ));
      }
    }
  }

  return Ok(());
}

fn set_current(path: &str) -> bool {
  env::set_current_dir(path).is_ok()
}
