use nix::sys::signal;
use nix::sys::signal::SaFlags;
use nix::sys::signal::{sigaction, SigAction, SigHandler, SigSet};

extern "C" fn handle_signal(_signam:i32) {}

pub fn signal_action() {
  let sa = SigAction::new(
    SigHandler::Handler(handle_signal),
    SaFlags::SA_RESETHAND,
    SigSet::empty(),
  );
  unsafe { sigaction(signal::SIGINT, &sa) }.unwrap();
}
