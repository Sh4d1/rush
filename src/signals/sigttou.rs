extern crate nix;
use self::nix::sys::signal;


pub fn init_sigttou() {
    let sig_action = signal::SigAction::new(
        signal::SigHandler::SigIgn,
        signal::SaFlags::empty(),
        signal::SigSet::empty()
    );

    unsafe { signal::sigaction(signal::SIGTTOU , &sig_action).expect("failed sigaction"); }
}
