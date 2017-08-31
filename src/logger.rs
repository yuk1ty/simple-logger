pub struct Logger {
    setting_level: NotificationLevel,
}

trait Logging {

    fn trace(&self, message: &'static str);

    fn debug(&self, message: &'static str);

    fn info(&self, message: &'static str);

    fn warn(&self, message: &'static str);

    fn error(&self, message: &'static str);
}

impl Logger {
    pub fn init(level: NotificationLevel) -> Self {
        Logger {
            setting_level: level,
        }
    }

    fn message(&self, message: &'static str, level: NotificationLevel) {
        let token = match level {
            NotificationLevel::Trace => "TRACE",
            NotificationLevel::Debug => "DEBUG",
            NotificationLevel::Info => "INFO",
            NotificationLevel::Warn => "WARN",
            NotificationLevel::Error => "ERROR",
        };
        println!("[{}] {}", token, message);
    }
}

impl Default for Logger {
    fn default() -> Self {
        Logger {
            setting_level: NotificationLevel::Trace
        }
    }
}

impl Logging for Logger {

    fn trace(&self, message: &'static str) {
        self.message(message, NotificationLevel::Trace);
    }

    fn debug(&self, message: &'static str) {
        self.message(message, NotificationLevel::Debug);
    }

    fn info(&self, message: &'static str) {
        self.message(message, NotificationLevel::Info);
    }

    fn warn(&self, message: &'static str) {
        self.message(message, NotificationLevel::Warn);
    }

    fn error(&self, message: &'static str) {
        self.message(message, NotificationLevel::Error);
    }
}

#[derive(Debug, PartialEq)]
pub enum NotificationLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
}

#[cfg(test)]
mod test {
    use super::Logger;
    use super::Logging;
    use super::NotificationLevel;

    #[test]
    fn initialize() {
        let logger = Logger::init(NotificationLevel::Trace);
        logger.trace("trace");
        assert_eq!(logger.setting_level, NotificationLevel::Trace);
    }
}