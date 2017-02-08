extern crate nix;
extern crate exec;
use self::nix::unistd::{fork, ForkResult, setpgid, getpid, tcsetpgrp, execvp};
use self::nix::sys::wait::*;
use self::nix::Error;
use self::nix::Errno;

use std::process;
use job_manager::*;
use command::command::CommandLine;
use functions::*;

pub fn parse(command: String) -> i8 {


    let command_line = CommandLine::new(command);
    let mut command = command_line.get_command();

    if command.is_empty() {
        return 0;
    } else if command.starts_with("cd") {
        return cd::change_dir(command.split_off(2).trim().to_string());
    } else if command.starts_with("fg") {
        return fg::fg(command.split_off(2).trim().to_string());
    } else if command.starts_with("bg") {
        return bg::bg(command.split_off(2).trim().to_string());
    } else if command.starts_with("jobs") {
        return jobs::jobs(command.split_off(4).trim().to_string());
    }
    execute(command_line)

}

pub fn execute(command: CommandLine) -> i8 {


    let mut err_code = 0;



    match fork() {
        Ok(ForkResult::Child)  => {
            use std::ffi::CString;
            let args = command.get_command();
            //let args_c: Vec<Vec<u8>> = args.trim().split(' ').collect().iter().map(|x| CString::new(x)).collect();
            let args: Vec<&str> = args.trim().split(' ').collect();
            let args_c: Vec<CString> = args.iter().map(|x| CString::new(x.as_bytes()).unwrap()).collect();
            let args = args.as_slice();
            let args_c = args_c.as_slice();


            setpgid(getpid(), getpid()).expect("setpgid failed");
            let _ = execvp(&args_c[0], &args_c[0..]);
            println!("rush: unknown command {}", &args[0]);
            process::exit(1);
        },
        Ok(ForkResult::Parent{child})  => {
            tcsetpgrp(1, child).expect("tcsetpgrp failed");
            if !command.get_bg() {
                JOB.lock().unwrap().set_active(child, command.get_command().to_owned());
                err_code = wait(child, command.get_command());
            } else {
                JOB.lock().unwrap().push(child, command.get_command().to_owned(), State::Running);
            }
            JOB.lock().unwrap().set_active(-1, "".to_owned());
            tcsetpgrp(1, getpid()).expect("tcsetpgrp failed");
        },
        Err(_) => (err_code = 1)

    }
    err_code
}

pub fn wait(pid: i32, name: String) -> i8 {
    use self::nix::sys::signal;

    let mut err_code = 0;
    let mut wait_pid_flags = WUNTRACED;
    wait_pid_flags.insert(WCONTINUED);
    loop {
        match waitpid(pid, Some(wait_pid_flags)) {
            Ok(WaitStatus::StillAlive) => (),
            Ok(WaitStatus::Continued(_)) => (),
            Ok(WaitStatus::Exited(_, code)) => { err_code = code; break}
            Ok(WaitStatus::Signaled(_, self::nix::sys::signal::Signal::SIGINT, _)) => break,
            ////Ok(WaitStatus::Signaled(a, self::nix::sys::signal::Signal::SIGTERM, _)) => println!("a"),
            Ok(WaitStatus::Stopped(_, self::nix::sys::signal::Signal::SIGTTOU)) => {
                tcsetpgrp(1, pid).expect("tcsetpgrp failed");
                signal::kill(pid, signal::SIGCONT).expect("sigcont failed");
            },
            Ok(WaitStatus::Stopped(_, self::nix::sys::signal::Signal::SIGSTOP)) => {signal::kill(pid, signal::SIGCONT).expect("sigcont failed");  println!("stop {}", pid);},
            Ok(WaitStatus::Stopped(_, self::nix::sys::signal::Signal::SIGTTIN)) => {
                //let mut buffer = String::new();
                //io::stdin().read_to_string(&mut buffer).unwrap();
                tcsetpgrp(1, pid).expect("tcsetpgrp failed");
                //println!("lol");
                signal::kill(pid, signal::SIGCONT).expect("sigcont failed");
            },
            Ok(WaitStatus::Stopped(_, self::nix::sys::signal::Signal::SIGTSTP)) => {

                JOB.lock().unwrap().push(pid, name.to_owned(), State::Stopped);
                println!("{} has stopped", name);
                break;
            },
            ////Ok(WaitStatus::Stopped(a, self::nix::sys::signal::Signal::SIGTERM)) => (),
            //Ok(WaitStatus::Stopped(a, b)) => println!("{:?} {:?}", a, b),
            //Ok(WaitStatus::Signaled(a, b, c)) => println!("{:?} {:?} {:?}", a, b, c),
            Err(Error::Sys(Errno::EINTR)) => (),
            Err(a) => {println!("{:?}", a); err_code = 1;break},
            a => {println!("{:?}", a); break},
            //Ok(a) => { println!("{:?}", a)}
        }
    }
    err_code
}
