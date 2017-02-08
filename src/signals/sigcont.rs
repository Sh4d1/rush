extern crate nix;
use self::nix::sys::signal;


extern fn handle_sigcont(_:i32) {
  println!("Paused!");
}

pub fn init_sigcont() {
    let sig_action = signal::SigAction::new(
        signal::SigHandler::Handler(handle_sigcont),
        signal::SaFlags::empty(),
        signal::SigSet::empty()
    );
    unsafe { signal::sigaction(signal::SIGCONT, &sig_action).expect("failed sigaction"); }
}
