use nix::unistd::*;

use super::process::Process;

impl Process {
  pub(crate) fn pipe_first_connect(&self) -> Result<(), String> {
    match self.get_pipe(self.len_pipes()) {
      Some(pipes) => {
        match dup2(pipes.1, 1) {
          Ok(_) => {}
          Err(_) => {
            return Err(format!("pipe first dup2 error"));
          }
        }

        match self.close_pipe(pipes) {
          Ok(_) => {}
          Err(_) => {
            return Err(format!("pipe first close error"));
          }
        }
      }
      None => {
        return Err(format!("pipe first missing"));
      }
    }

    return Ok(());
  }

  pub(crate) fn pipe_end_connect(&self) -> Result<(), String> {
    match self.get_pipe(self.len_pipes()) {
      Some(pipes) => {
        match dup2(pipes.0, 0) {
          Ok(_) => {}
          Err(_) => {
            return Err(format!("pipe close dup2 error"));
          }
        }

        match self.close_pipe(pipes) {
          Ok(_) => {}
          Err(_) => {
            return Err(format!("pipe end close error"));
          }
        }
      }

      None => {
        return Err(format!("pipe end missing"));
      }
    }

    return Ok(());
  }

  pub(crate) fn pipe_route_connect(&self) -> Result<(), String> {
    match self.get_pipe(self.len_pipes() - 1) {
      Some(pipes) => {
        match dup2(pipes.0, 0) {
          Ok(_) => {}

          Err(_) => {
            return Err(format!("pipe route dup2 error"));
          }
        }

        match self.close_pipe(pipes) {
          Ok(_) => {}
          Err(_) => {
            return Err(format!("pipe route close error"));
          }
        }
      }

      None => {
        return Err(format!("pipe route missinag"));
      }
    }

    match self.get_pipe(self.len_pipes()) {
      Some(pipes) => {
        match dup2(pipes.1, 1) {
          Ok(_) => {}
          Err(_) => {
            return Err(format!("pipe route dup2 error"));
          }
        }
        match self.close_pipe(pipes) {
          Ok(_) => {}
          Err(_) => {
            return Err(format!("pipe route close error"));
          }
        }
      }
      None => {
        return Err(format!("pipe route missinag"));
      }
    }

    return Ok(());
  }

  pub(crate) fn pearent_connect_end(&mut self) -> Result<(), String> {
    match self.get_pipe(0) {
      Some(pipes) => match self.close_pipe(pipes) {
        Ok(_) => {}
        Err(_) => {
          return Err(format!("pearent end close error"));
        }
      },
      None => {
        return Err(format!("pipe missing"));
      }
    }

    self.deque_pipe();
    return Ok(());
  }

  pub(crate) fn close_pipe(&self, pipe: &(i32, i32)) -> Result<(), String> {
    match close(pipe.0) {
      Ok(_) => {}
      Err(_) => {
        return Err(format!("pipe close error"));
      }
    }
    match close(pipe.1) {
      Ok(_) => {}
      Err(_) => {
        return Err(format!("pipe clsoe error"));
      }
    }
    return Ok(());
  }
}
