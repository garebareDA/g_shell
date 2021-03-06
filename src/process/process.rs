use super::super::parser;

pub struct Process {
  run_command: parser::parser::CommandParse,
  pub pipes: Vec<(i32, i32)>,
  pub process: Vec<nix::unistd::Pid>,
}

impl Process {
  pub fn new(command: &parser::parser::CommandParse) -> Self {
    Self {
      run_command: command.clone(),
      pipes: Vec::new(),
      process: Vec::new(),
    }
  }

  pub fn get_run_command(&self) -> &parser::parser::CommandParse {
    &self.run_command
  }

  pub fn push_pipe(&mut self, pipe: (i32, i32)) {
    self.pipes.push(pipe);
  }

  pub fn len_pipes(&self) -> usize {
    self.pipes.len() - 1
  }

  pub fn get_pipe(&self, index: usize) -> Option<&(i32, i32)> {
    self.pipes.get(index)
  }

  pub fn is_empty_pipes(&self) -> bool {
    self.pipes.is_empty()
  }

  pub fn pop_pipes(&mut self) {
    if self.len_pipes() == 0 {
      return;
    }
    self.pipes.pop();
  }

  pub fn deque_pipe(&mut self) {
    if self.len_pipes() == 0 {
      return;
    }
    self.pipes.remove(0);
  }

  pub fn push_process(&mut self, pid: nix::unistd::Pid) {
    self.process.push(pid);
  }

  pub fn get_process(&self) -> &Vec<nix::unistd::Pid> {
    &self.process
  }
}
