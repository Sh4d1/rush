extern crate nix;

use job_manager::JOB;
use self::nix::sys::signal::kill;
use self::nix::sys::signal;
use command::execute;

pub fn fg(args: String) -> i8 {
    let mut err_code = 0;
    let size = args.split_whitespace().count();

    match size {
        0 => {
            if JOB.lock().unwrap().get_size() == 0 {
                println!("No jobs running");
                err_code = 1;
            } else {
                let (pid, name) = JOB.lock().unwrap().pop();
                println!("Send job {} to foreground", name);
                kill(pid, signal::SIGCONT).expect("sigcont failed");
                JOB.lock().unwrap().set_active(pid, name.to_owned());
                execute::wait(pid, name);
            }
        }
        _ => {
            err_code = 1;
            println!("Too many arguments for cd command");
        },
    }
    err_code
}
