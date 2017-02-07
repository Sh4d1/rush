#![cfg(not(test))]
extern crate rushlib;
extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::completion::FilenameCompleter;
use std::process;
extern crate core;
use rushlib::prompt::Prompt;
use rushlib::command::execute;
extern crate ansi_term;
use ansi_term::Colour::Fixed;


fn main() {
    let mut exit_code = 0;
    let mut prompt = Prompt::new();
    let mut rl = rustyline::Editor::<FilenameCompleter>::new();
    rl.set_completer(Some(rustyline::completion::FilenameCompleter::new()));
    loop {
        prompt.update_cwd();
        let readline = rl.readline(prompt.print().as_str());
        match readline {
            Ok(line) => {
                let line = line.trim().to_string();
                rl.add_history_entry(line.as_str());

                if line == "exit" {
                    exit_code = 1;
                    break
                }
                //execute::parse(line)
                prompt.update_error(execute::parse(line));
            },
            Err(ReadlineError::Interrupted) => {
                println!("{}", Fixed(221).on(Fixed(124)).paint("^C"));
            },
            Err(ReadlineError::Eof) => {
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                exit_code = 1;
                break
            }
        }

    }
    process::exit(exit_code);
}
