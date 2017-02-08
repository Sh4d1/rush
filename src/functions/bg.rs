extern crate nix;
use job_manager::*;
use self::nix::unistd::{getpid, tcsetpgrp};
use self::nix::sys::signal::kill;
use self::nix::sys::signal;

pub fn bg(args: String) -> i8 {
    let mut err_code = 0;
    let size = args.split_whitespace().count();

    match size {
        0 => {
            if JOB.lock().unwrap().get_size() == 0 {
                println!("bg: No running jobs");
                err_code = 1;
            } else {
                let a = JOB.lock().unwrap().pop_stopped();
                match a {
                    Some((pid, name, State::Stopped)) => {
                        println!("Send job {} to background : {}", name, pid);
                        kill(-pid, signal::SIGCONT).expect("sigcont failed");
                        JOB.lock().unwrap().push(pid, name, State::Running);
                        tcsetpgrp(1, getpid()).expect("tcsetpgrp failed");
                    },
                    _ => {
                        println!("bg: No stopped jobs");
                        err_code = 1;
                    }
                }
                //let Some()(pid, name, state) = JOB.lock().unwrap().pop_stopped();
                //match JOB.lock().unwrap().pop_stopped() {
                //    Some((pid, name, _)) => {
                //        println!("Send job {} to background", name);
                //        tcsetpgrp(1, getpid());
                //        JOB.lock().unwrap().push(pid, name, State::Running);
                //        kill(pid, signal::SIGCONT).expect("sigcont failed");
                //    },
                //    None => {
                //        println!("bg: No stopped jobs");
                //        err_code = 1;
                //    }
                //
                //}
                //match state {
                //    State::Stopped => {
                //        println!("Send job {} to background", name);
                //        kill(pid, signal::SIGCONT).expect("sigcont failed");
                //        JOB.lock().unwrap().push(pid, name, State::Running);
                //        tcsetpgrp(1, getpid());
                //    },
                //    _ => {
                //        println!("bg: No stopped jobs");
                //        err_code = 1;
                //    },
                //}
            }
        }
        _ => {
            err_code = 1;
            println!("Too many arguments for bg command");
        },
    }
    err_code
}
