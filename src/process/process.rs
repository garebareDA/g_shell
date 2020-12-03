use super::super::parser;

pub struct Process {
  run_command:parser::parser::CommandParse
}

impl Process {
  pub fn new(command:&parser::parser::CommandParse) -> Self {
    Self {
      run_command:command.clone(),
    }
  }

  pub fn get_run_command(&self) -> &parser::parser::CommandParse {
    &self.run_command
  }
}