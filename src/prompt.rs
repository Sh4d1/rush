
use std::env::{current_dir, home_dir};
use std::env;
use ansi_term::Colour::Purple;
use ansi_term::Colour::Green;
use ansi_term::Colour::Red;

pub struct Prompt {
    user: String,
    cwd: String,
    error: i8,
}
impl Prompt {
    pub fn new() -> Prompt {
        let mut prompt = Prompt {
            user: match env::var("USER") {
                Ok(val) => Purple.paint(val).to_string(),
                Err(_) => Purple.paint("?").to_string(),
            },
            cwd: "".to_string(),
            error: 0,
        };
        prompt.update_cwd();
        prompt
    }

    pub fn print(&self) -> String {
        if self.error != 0 {
            format!("{} {}:{} > ", Red.paint(format!("\u{2718} {}", self.error)).to_string(), self.user, self.cwd)
        } else {
            format!("{}:{} > ", self.user, self.cwd)
        }
    }

    pub fn update_cwd(&mut self) {
        let mut cwd = current_dir().unwrap().as_path().to_str().expect("Failed : path -> str").to_string();
        let homedir = home_dir().unwrap().as_path().to_str().expect("Failed : path -> str").to_string();

        self.cwd = if cwd.starts_with(homedir.as_str()) {
            Green.paint(format!("~{}", cwd.split_off(homedir.len()))).to_string()
            } else { Green.paint(cwd).to_string() };
    }

    pub fn update_error(&mut self, error: i8) {
        self.error = error;
    }

}


#[cfg(test)]
mod tests{

    #[allow(unused_imports)]
    use std::env::{current_dir,home_dir};
    use super::*;

    #[test]
    fn updated_cwd() {
        let mut testp = Prompt::new();
        testp.update_cwd();
        assert_eq!(testp.get_cwd(), current_dir().ok()
                   .expect("Couldn't get current directory").as_path()
                   .to_str()
                   .expect("Failed to go to string").to_owned());
    }
}
