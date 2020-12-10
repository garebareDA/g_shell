use super::process::Process;
use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use nix::unistd::*;

impl Process {
  pub(crate) fn redirect(&self, path: &str) -> Result<(), String> {
    let mode = Mode::S_IRWXU;
    let flag = OFlag::O_CREAT | OFlag::O_WRONLY | OFlag::O_TRUNC;
    match self.open_file(path, flag, mode) {
      Ok(_) => {
        return Ok(());
      }
      Err(e) => {
        return Err(e);
      }
    }
  }

  pub(crate) fn over_redirect(&self, path: &str) -> Result<(), String> {
    let mode = Mode::S_IRWXU;
    let flag = OFlag::O_CREAT | OFlag::O_WRONLY | OFlag::O_APPEND;
    match self.open_file(path, flag, mode) {
      Ok(_) => {
        return Ok(());
      }
      Err(e) => {
        return Err(e);
      }
    }
  }

  fn open_file(&self, path: &str, flag: OFlag, mode: Mode) -> Result<(), String> {
    match open(path, flag, mode) {
      Ok(file) => match dup2(file, 1) {
        Ok(_) => match close(file) {
          Ok(_) => {}
          Err(_) => {
            return Err(format!("file clse error"));
          }
        },
        Err(_) => {
          return Err(format!("file dup2 error"));
        }
      },
      Err(_) => {
        return Err(format!("file open error"));
      }
    }
    return Ok(());
  }
}
