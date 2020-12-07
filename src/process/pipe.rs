use nix::unistd::*;
use std::process::exit;

use super::process::Process;

impl Process {
  pub fn pipe_first_connect(&self) {
    match dup2(self.get_pipe(self.len_pipes()).1, 1) {
      Ok(_) => {}
      Err(_) => {
        println!("first pipe error");
        exit(-1);
      }
    }

    match close(self.get_pipe(self.len_pipes()).0) {
      Ok(_) => {}
      Err(_) => {
        println!("first pipe close error");
        exit(-1);
      }
    }

    match close(self.get_pipe(self.len_pipes()).1) {
      Ok(_) => {}
      Err(_) => {
        println!("first pipe close error");
        exit(-1);
      }
    }
  }

  pub fn pipe_end_connect(&self) {
    match dup2(self.get_pipe(self.len_pipes()).0, 0) {
      Ok(_) => {}

      Err(_) => {
        println!("pipe error");
        exit(-1);
      }
    }

    match close(self.get_pipe(self.len_pipes()).0) {
      Ok(_) => {}
      Err(_) => {
        println!("pipe close error");
        exit(-1);
      }
    }

    match close(self.get_pipe(self.len_pipes()).1) {
      Ok(_) => {}
      Err(_) => {
        println!("pipe close error");
        exit(-1);
      }
    }
  }

  pub fn pipe_route_connect(&self) {
    match dup2(self.get_pipe(self.len_pipes() - 1).0, 0) {
      Ok(_) => {}

      Err(_) => {
        println!("pipe error");
        exit(-1);
      }
    }

    match dup2(self.get_pipe(self.len_pipes()).1, 1) {
      Ok(_) => {}

      Err(_) => {
        println!("pipe error");
        exit(-1);
      }
    }

    match close(self.get_pipe(self.len_pipes() - 1).0) {
      Ok(_) => {}
      Err(_) => {
        println!("pipe close error");
        exit(-1);
      }
    }

    match close(self.get_pipe(self.len_pipes() - 1).1) {
      Ok(_) => {}
      Err(_) => {
        println!("pipe close error");
        exit(-1);
      }
    }

    match close(self.get_pipe(self.len_pipes()).0) {
      Ok(_) => {}
      Err(_) => {
        println!("pipe close error");
        exit(-1);
      }
    }

    match close(self.get_pipe(self.len_pipes()).1) {
      Ok(_) => {}
      Err(_) => {
        println!("pipe close error");
        exit(-1);
      }
    }
  }

  pub fn pearent_connect_end(&self) {
    match close(self.get_pipe(self.len_pipes()).0) {
      Ok(_) => {}
      Err(_) => {
        println!("pipe close error");
        exit(-1);
      }
    }

    match close(self.get_pipe(self.len_pipes()).1) {
      Ok(_) => {}
      Err(_) => {
        println!("pipe close error");
        exit(-1);
      }
    }
  }
}
