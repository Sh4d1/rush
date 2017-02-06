
use std::env::{current_dir, home_dir};
use ansi_term::Colour::Purple;
use ansi_term::Colour::Green;


pub struct Prompt {
    user: String,
    pwd: String,
}
impl Prompt {
    pub fn new() -> Prompt {
        let mut prompt = Prompt {
            user: Purple.paint("patrik").to_string(),
            pwd: "".to_string(),
        };
        prompt.update_pwd();
        prompt
    }

    pub fn print(&self) -> String {
        format!("{}:{} > ", self.user, self.pwd)
    }

    pub fn update_pwd(&mut self) {
        let mut pwd = current_dir().unwrap().as_path().to_str().expect("Failed : path -> str").to_string();
        let homedir = home_dir().unwrap().as_path().to_str().expect("Failed : path -> str").to_string();

        self.pwd = if pwd.starts_with(homedir.as_str()) {
            Green.paint(format!("~{}", pwd.split_off(homedir.len()))).to_string()
            } else { Green.paint(pwd).to_string() };
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
