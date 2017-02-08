extern crate nix;
use self::nix::sys::signal;
use self::nix::sys::wait::*;
use self::nix::sys::signal::kill;
use job_manager::JOB;


extern fn handle_sigint(_:i32) {
    let wait_pid_flags = WNOHANG;
    let _ = waitpid(-1, Some(wait_pid_flags));
    match JOB.lock().unwrap().get_active() {
        (a,_) if a <= -1 => (),
        (a,_) => match kill(a, signal::SIGINT) {
            Err(e) => println!("Error handling SIGINT : {}", e),
            _ => println!("\n")
            },
    }
}

pub fn init_sigint() {
    let sig_action = signal::SigAction::new(
        signal::SigHandler::Handler(handle_sigint),
        signal::SaFlags::empty(),
        signal::SigSet::empty()
    );
    unsafe { signal::sigaction(signal::SIGINT , &sig_action).expect("failed sigaction"); }
}
