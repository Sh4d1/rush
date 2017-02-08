extern crate shellexpand;
extern crate nix;
extern crate exec;
use self::nix::unistd::{fork, ForkResult, setpgid, getpid};
use self::nix::sys::wait::*;
use self::nix::Error;
use self::nix::Errno;

use std::env;
use std::process;
use std::borrow::Cow;
use std::path::{Path,PathBuf};
use job_manager::JOB;

use command::command::CommandLine;
use functions::*;



fn home_dir() -> Option<PathBuf> { Some(Path::new(env::var("HOME").expect("no $HOME var in env").as_str()).into()) }

fn context(s: &str) -> Result<Option<Cow<'static, str>>, env::VarError> {

    match env::var(s) {
        Ok(value) => Ok(Some(value.into())),
        Err(env::VarError::NotPresent) => Ok(Some("".into())),
        Err(e) => Err(e)
    }
}



pub fn parse(mut command: String) -> i8 {

    command = match shellexpand::full_with_context(command.as_str(), home_dir, context) {
        Ok(s) => s.to_string(),
        _ => "".to_string()
    };


    if command.is_empty() {
        return 0;
    } else if command.starts_with("cd") {
        return cd::change_dir(command.split_off(2).trim().to_string());
    } else if command.starts_with("fg") {
        return fg::fg(command.split_off(2).trim().to_string());
    }
    let command = CommandLine::new(command);
    execute(command)

}

pub fn execute(command: CommandLine) -> i8 {


    let mut err_code = 0;



    match fork() {
        Ok(ForkResult::Child)  => {
            let args = command.get_command();
            let args: Vec<&str> = args.trim().split(' ').collect();
            let args = args.as_slice();
            setpgid(getpid(), getpid()).expect("setpgid failed");
            let _ = exec::Command::new(&args[0]).args(&args[1.. ]).exec();
            println!("rush: unknown command {}", &args[0]);
            process::exit(1);
        },
        Ok(ForkResult::Parent{child})  => {
            JOB.lock().unwrap().set_active(child, command.get_command().to_owned());
            if !command.get_bg() {
                err_code = wait(child, command.get_command());
                //waitpid(child, None);
            }
            JOB.lock().unwrap().set_active(-1, "".to_owned());
        },
        Err(_) => ()

    }
    err_code
}

pub fn wait(pid: i32, name: String) -> i8 {
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
            Ok(WaitStatus::Stopped(_, self::nix::sys::signal::Signal::SIGTSTP)) => {
                JOB.lock().unwrap().push(pid, name.to_owned());
                println!("{} has stopped", name);
                break;
            },
            ////Ok(WaitStatus::Stopped(a, self::nix::sys::signal::Signal::SIGTERM)) => (),
            //Ok(WaitStatus::Stopped(a, b)) => println!("{:?} {:?}", a, b),
            //Ok(WaitStatus::Signaled(a, b, c)) => println!("{:?} {:?} {:?}", a, b, c),
            Err(Error::Sys(Errno::EINTR)) => (),
            Err(a) => {println!("{:?}", a); break},
            _ => break,
            //Ok(a) => { println!("{:?}", a)}
        }
    }
    err_code
}
