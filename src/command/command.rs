extern crate shellexpand;
use std::path::{Path,PathBuf};
use std::env;
use std::borrow::Cow;


pub struct CommandLine {
    line: String,
    bg: bool,
}

impl CommandLine {
    pub fn new(s: String) -> CommandLine {
        let bg = s.contains("&");
        //let s = s.replace("&", "");
        let command = match shellexpand::full_with_context(s.replace("&", "").as_str(), home_dir, context) {
            Ok(s) => s.to_string(),
            _ => "".to_string()
        };
        CommandLine { line: command, bg: bg}
    }

    pub fn get_command(&self) -> String {
        self.line.to_string()
    }

    pub fn get_bg(&self) -> bool {
        self.bg
    }
}

fn home_dir() -> Option<PathBuf> { Some(Path::new(env::var("HOME").expect("no $HOME var in env").as_str()).into()) }

fn context(s: &str) -> Result<Option<Cow<'static, str>>, env::VarError> {

    match env::var(s) {
        Ok(value) => Ok(Some(value.into())),
        Err(env::VarError::NotPresent) => Ok(Some("".into())),
        Err(e) => Err(e)
    }
}
