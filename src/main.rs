#![cfg(not(test))]
extern crate rushlib;
extern crate rustyline;

use rustyline::error::ReadlineError;
extern crate core;
use rushlib::prompt::Prompt;
use rushlib::command::execute;
extern crate ansi_term;
use ansi_term::Colour::Fixed;


fn main() {
    let mut prompt = Prompt::new();
    let mut rl = rustyline::Editor::<()>::new();
    loop {
        prompt.update_cwd();
        let readline = rl.readline(prompt.print().as_str());
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());


                execute::parse(line);
                //if !output.is_empty() {
                //    println!("{}",output.trim());
                //}
            },
            Err(ReadlineError::Interrupted) => {
                println!("{}", Fixed(221).on(Fixed(124)).paint("^C"));
            },
            Err(ReadlineError::Eof) => {
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }

    }
    println!("Number of commands: {}", rl.get_history().len());
}
