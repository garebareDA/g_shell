use nix::sys::signal;
use nix::sys::signal::SaFlags;
use nix::sys::signal::{sigaction, SigAction, SigHandler, SigSet};

use super::process;

extern "C" fn handle_signal(_signam: i32) {
  println!();
}

impl process::Process {
  pub(crate) fn signal_action(&self) {
    let sa = SigAction::new(
      SigHandler::Handler(handle_signal),
      SaFlags::SA_RESETHAND,
      SigSet::empty(),
    );
    unsafe { sigaction(signal::SIGINT, &sa) }.unwrap();
  }
}