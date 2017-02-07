extern crate shellexpand;
extern crate nix;
extern crate exec;
use self::nix::unistd::{fork, ForkResult, };
use self::nix::sys::wait::*;
use self::nix::Error;
use self::nix::Errno;
use self::nix::sys::signal::Signal;

use std::env;

use std::borrow::Cow;
use std::path::{Path,PathBuf};


use command::command::CommandLine;
use functions::cd;

fn home_dir() -> Option<PathBuf> { Some(Path::new(env::var("HOME").expect("no $HOME var in env").as_str()).into()) }

fn context(s: &str) -> Result<Option<Cow<'static, str>>, env::VarError> {

    match env::var(s) {
        Ok(value) => Ok(Some(value.into())),
        Err(env::VarError::NotPresent) => Ok(Some("".into())),
        Err(e) => Err(e)
    }
}



pub fn parse(mut command: String) {
    command = command.trim().to_string();

    command = match shellexpand::full_with_context(command.as_str(), home_dir, context) {
        Ok(s) => s.to_string(),
        _ => "".to_string()
    };


    if command.is_empty() {
        return;
    } else if command.starts_with("cd") {
        cd::change_dir(command.split_off(2).trim().to_string());
        return;
    }
    let command = CommandLine::new(command);
    execute(command);

}

pub fn execute(command: CommandLine) {


    match fork() {
        Ok(ForkResult::Child)  => {
            let args = command.get_command();
            let args: Vec<&str> = args.trim().split(' ').collect();
            let args = args.as_slice();

            let _ = exec::Command::new(&args[0]).args(&args[1.. ]).exec();
            println!("rush: unknown command {}", &args[0]);

        },
        Ok(ForkResult::Parent{child})  => {
            if !command.get_bg() {
                loop {
                    match waitpid(child, None) {
                        Ok(WaitStatus::StillAlive) => (),
                        //Ok(WaitStatus::Signaled(_, Signal::SIGTERM, _)) => break,
                        Err(Error::Sys(Errno::EINTR)) => (),
                        a => {println!("{:?}", a); break}
                    }
                }
                //waitpid(child, None);
            }

        },
        Err(_) => ()
    }
    //if let Ok(mut child) = Command::new(&args[0]).args(&args[1.. ]).spawn() {
    //    if !command.get_bg() {
    //        child.wait().expect("Couldn't wait for process.");
    //    }
    //} else {
    //    println!("Unknown command : {}", &args[0]);
    //}

}
