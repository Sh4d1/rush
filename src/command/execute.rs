use std::process::Command;

use std::process::Output;
use std::process::*;
use command::command::CommandLine;

pub fn parse(command: String) {
    if command.is_empty() {
        return;
    }
    let command = CommandLine::new(command);
    execute(command);

}

pub fn execute(command: CommandLine) {
    let args = command.get_command();
    let args: Vec<&str> = args.trim().split(' ').collect();
    let args = args.as_slice();

    //let mut exec = Command::new(&args[0]).args(&args[1.. ]); //.spawn().expect("failed to execute child"));

    if let Ok(mut child) = Command::new(&args[0]).args(&args[1.. ]).spawn() {
        if !command.get_bg() {
            child.wait().expect("Couldn't wait for process.");
        }
    } else {
        println!("Unknown command : {}", &args[0]);
    }
    //if let Ok(exec) = exec.spawn() {
    //
    //}
    //.spawn().expect("failed to execute child");



    //if command.get_bg() {
    //
    //} else {
    //    if args.len() > 1 {
    //        Command::new(&args[0]).args(&args[1.. ]).output().ok()
    //    } else if args.len() == 1{
    //        Command::new(&args[0]).output().ok()
    //    } else {
    //        Command::new("").output().ok()
    //    }
    //}



}
