pub struct CommandLine {
    line: String,
    bg: bool,
}

impl CommandLine {
    pub fn new(s: String) -> CommandLine {
        let bg = s.contains("&");
        let s = s.trim().to_string().replace("&", "");
        CommandLine { line: s, bg: bg}
    }

    pub fn get_command(&self) -> String {
        self.line.to_string()
    }

    pub fn get_bg(&self) -> bool {
        self.bg
    }
}
