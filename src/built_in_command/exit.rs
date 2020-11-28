use std::process::exit;

use super::super::parser::parser::CommandParse;

pub fn run_exit(command:&CommandParse) -> Result<(), String> {
  if command.get_command() == "exit" {
    if command.get_index() == 1 {
      exit(1);
    }else{
      return Err(format!("exit has no subcommands and options"));
    }
  }
  return Ok(());
}