enum LogLevel {
    Notice,
    Error,
}

impl LogLevel {
    fn prefix(&self) -> String {
        match self {
            Self::Notice => String::from(""),
            Self::Error => String::from("[ERROR]"),
        }
    }
}

pub struct Printer {}

impl Printer {
    pub fn new() -> Self {
        Self {}
    }

    fn print(&self, level: LogLevel, msg: &str) {
        let prefix = level.prefix();
        if prefix.is_empty() {
            println!("{}", msg)
        } else {
            println!("{}: {}", level.prefix(), msg)
        }
    }

    pub fn notice(&self, msg: &str) {
        self.print(LogLevel::Notice, msg)
    }

    pub fn error(&self, msg: &str) {
        self.print(LogLevel::Error, msg)
    }
}

impl Default for Printer {
    fn default() -> Self {
        Self::new()
    }
}
