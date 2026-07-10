#[derive(Debug)]
pub enum InputMode {
    Normal,
    Command,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Search,
    Kill,
    RefreshTime,
    Unknown,
}

impl Command {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "search" => Command::Search,
            "kill" => Command::Kill,
            "refresh" => Command::RefreshTime,

            _ => Command::Unknown,
        }
    }
}
