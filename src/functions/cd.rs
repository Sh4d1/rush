use std::env;
use std::path::{Path,PathBuf};




pub fn change_dir(mut args: String) {
    let size = args.split_whitespace().count();

    match size {
        x if x > 1 => println!("Too many arguments for cd command"),
        0 => {
            env::set_current_dir(Path::new(env::var("HOME").expect("no $HOME var in env").as_str()));
        }
        _ => {
            let mut path = PathBuf::new();
            let home_var = match env::var("HOME") {
                Ok(val) => val,
                Err(_) => "/".to_string(),
            };
            if args.starts_with("~") {
                path.push(Path::new(home_var.as_str()));
                path.push(Path::new(args.split_off(1).as_str()));
            } else {
                if Path::new(args.as_str()).is_relative() {
                    path.push(env::current_dir().unwrap().as_path());
                }
                path.push(Path::new(args.as_str()));
            }

            if path.exists(){
                match env::set_current_dir(path) {
                    Err(e) => println!("Error: {}", e),
                    _ => (),
                }

            } else {
                println!("{} does not exist", args);
            }

        },
    }

}
