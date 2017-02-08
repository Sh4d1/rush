extern crate nix;
use self::nix::sys::signal;
use self::nix::sys::wait::*;
use self::nix::sys::signal::kill;
use job_manager::JOB;


extern fn handle_sigtstp(_: i32) {
    let wait_pid_flags = WNOHANG;
    let _ = waitpid(-1, Some(wait_pid_flags));
    match JOB.lock().unwrap().get_active() {
        (a,_) if a <= -1 => (),
        (a,_) => match kill(a, signal::SIGTSTP) {
            Err(e) => println!("Error handling SIGTSTP : {}", e),
            _ => ()
        },
    }
}

pub fn init_sigtstp() {
    let sig_action = signal::SigAction::new(
        signal::SigHandler::Handler(handle_sigtstp),
        signal::SaFlags::empty(),
        signal::SigSet::empty()
    );
    unsafe { signal::sigaction(signal::SIGTSTP , &sig_action).expect("failed sigaction"); }
}
