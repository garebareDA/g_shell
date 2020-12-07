use super::super::parser;

pub struct Process {
  run_command:parser::parser::CommandParse,
  pipes:Vec<(i32, i32)>
}

impl Process {
  pub fn new(command:&parser::parser::CommandParse) -> Self {
    Self {
      run_command:command.clone(),
      pipes:Vec::new(),
    }
  }

  pub fn get_run_command(&self) -> &parser::parser::CommandParse {
    &self.run_command
  }

  pub fn push_pipe(&mut self, pipe:(i32, i32)) {
    self.pipes.push(pipe);
  }

  pub fn len_pipes(&self) -> usize {
    self.pipes.len() - 1
  }

  pub fn get_pipe(&self, index: usize) -> (i32, i32) {
    self.pipes[index]
  }

  pub fn is_empty_pipes(&self) -> bool {
    self.pipes.is_empty()
  }
}