extern crate nix;
use job_manager::*;
use self::nix::unistd::{getpid, tcsetpgrp};
use self::nix::sys::signal::kill;
use self::nix::sys::signal;
use command::execute;

pub fn fg(args: String) -> i8 {
    let err_code: i8;
    let size = args.split_whitespace().count();

    match size {
        0 => {
            if JOB.lock().unwrap().get_size() == 0 {
                println!("fg: No jobs running");
                err_code = 1;
            } else {
                let (pid, name, state) = JOB.lock().unwrap().pop();
                println!("Send job {} to foreground", name);
                match state {
                    State::Stopped => kill(-pid, signal::SIGCONT).expect("sigcont failed"),
                    _ => (),
                }

                JOB.lock().unwrap().set_active(pid, name.to_owned());
                err_code = execute::wait(pid, name);
                tcsetpgrp(1, getpid()).expect("tcsetpgrp failed");
            }
        }
        _ => {
            err_code = 1;
            println!("Too many arguments for fg command");
        },
    }
    err_code
}
