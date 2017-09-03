pub struct Logger {
    setting_level: NotificationLevel,
}

trait Logging {
    fn trace(self, message: &'static str);

    fn debug(self, message: &'static str);

    fn info(self, message: &'static str);

    fn warn(self, message: &'static str);

    fn error(self, message: &'static str);
}

impl Logger {
    pub fn init(level: NotificationLevel) -> Self {
        Logger {
            setting_level: level,
        }
    }

    fn message(self, message: &'static str, level: NotificationLevel) {
        if level.lower_than(self.setting_level) {
            return;
        }

        let token = match &level {
            &NotificationLevel::Trace => "TRACE",
            &NotificationLevel::Debug => "DEBUG",
            &NotificationLevel::Info => "INFO",
            &NotificationLevel::Warn => "WARN",
            &NotificationLevel::Error => "ERROR",
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
    fn trace(self, message: &'static str) {
        self.message(message, NotificationLevel::Trace);
    }

    fn debug(self, message: &'static str) {
        self.message(message, NotificationLevel::Debug);
    }

    fn info(self, message: &'static str) {
        self.message(message, NotificationLevel::Info);
    }

    fn warn(self, message: &'static str) {
        self.message(message, NotificationLevel::Warn);
    }

    fn error(self, message: &'static str) {
        self.message(message, NotificationLevel::Error);
    }
}

#[derive(Debug, Copy, Clone)]
pub enum NotificationLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
}

impl NotificationLevel {
    fn lower_than(self, rhs: NotificationLevel) -> bool {
        rhs as i32 > self as i32
    }
}

#[cfg(test)]
mod test {
    use super::Logger;
    use super::Logging;
    use super::NotificationLevel;

    #[test]
    fn initialize() {
        // TODO implements appropriate test
        let logger = Logger::init(NotificationLevel::Trace);
        logger.trace("trace");
    }

    #[test]
    fn test_lower_than_notification_level() {
        let lhs = NotificationLevel::Trace;
        assert!(lhs.lower_than(NotificationLevel::Debug));
    }
}