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
                Err(e) => "/".to_string(),
            };
            //let mut home_var = env::var("HOME").expect("no $HOME var in env").as_str();
            if args.starts_with("~") {
                path.push(Path::new(home_var.as_str()));
                path.push(Path::new(args.split_off(1).as_str()));
            } else {
                path.push(Path::new(args.as_str()));
                if Path::new(args.as_str()).is_relative() {
                    path.push(env::current_dir().unwrap().as_path());
                }
                path.push(Path::new(args.as_str()));
            }

            if path.exists(){
                env::set_current_dir(path)
                    .expect("Failed to set current directory");
            } else {
                println!("{:?}", path);
                println!("{} does not exist", args);
            }

        },
    }
    //if args.split_whitespace().count() > 1 {
    //    println!("Too many arguments for cd command");
    //} else if args.split_whitespace().count() == 0 {
    //    env::set_current_dir(Path::new(env::var("HOME")
    //                                   .expect("no $HOME var in env").as_str()));
    //} else {
    //    let mut path = PathBuf::new();
    //    let home_path = Path::new(env::var("HOME")
    //                              .expect("no $HOME var in env").as_str());
    //    if args.starts_with("~") {
    //        path.push(home_path);
    //        path.push(Path::new(args.split_off(1).as_str()));
    //    } else {
    //        path.push(Path::new(args.as_str()));
    //    }
    //
    //    let mut path = path.as_path();
    //
    //    if path.is_relative() {
    //        path = home_path.join(path).as_path();
    //    }
    //    if path.exists(){
    //        env::set_current_dir(path)
    //            .expect("Failed to set current directory");
    //    } else {
    //        println!("{} does not exist", args);
    //    }
    //
    //}


}
