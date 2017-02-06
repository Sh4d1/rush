use std::process::Command;

use command::command::CommandLine;
use functions::cd;

pub fn parse(mut command: String) {
    command = command.trim().to_string();
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
    let args = command.get_command();
    let args: Vec<&str> = args.trim().split(' ').collect();
    let args = args.as_slice();


    if let Ok(mut child) = Command::new(&args[0]).args(&args[1.. ]).spawn() {
        if !command.get_bg() {
            child.wait().expect("Couldn't wait for process.");
        }
    } else {
        println!("Unknown command : {}", &args[0]);
    }

}
