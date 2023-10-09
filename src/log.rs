struct Logger {
    to: Option<Box<dyn std::io::Write>>,
}
static mut LOGGER: Logger = Logger { to: None };
#[derive(PartialEq, PartialOrd, Copy, Clone, Debug)]
pub enum Level {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}
static mut LEVEL: Level = Level::Info;

impl Logger {
    /// Logs a message at the specified level.
    pub fn log(&mut self, level: Level, s: &str) {
        if level < unsafe { LEVEL } {
            return;
        }
        if let Some(ref mut to) = self.to {
            let _ = to
                .write(
                    match level {
                        Level::Trace => format!("[TRACE] {}\n", s),
                        Level::Debug => format!("[DEBUG] {}\n", s),
                        Level::Info => format!("[INFO] {}\n", s),
                        Level::Warn => format!("[WARN] {}\n", s),
                        Level::Error => format!("[ERROR] {}\n", s),
                    }
                    .as_bytes(),
                )
                .unwrap();
        }
        // self.to.write(b"\n").unwrap();
    }
}
/// Initializes the logger.
///
/// # Examples
///
/// ```
/// use xlog_rs::log;
/// log::init(std::io::stdout(), log::Level::Trace);
/// assert_eq!(log::level(), log::Level::Trace);
/// ```
pub fn init(to: impl std::io::Write + 'static, level: Level) {
    // Box::new(to);
    unsafe {
        LOGGER = Logger {
            to: Some(Box::new(to)),
        };
        LEVEL = level;
    }
}
/// Initializes the logger with a file.
pub fn with_file<'a>(name: &str) {
    // Box::new(to);
    unsafe {
        LOGGER = Logger {
            to: Some(Box::new(
                std::fs::File::options()
                    .create(true)
                    .append(true)
                    .open(name)
                    .unwrap(),
            )),
        };
    }
}
/// Sets the log level.
///
/// # Examples
///
/// ```
/// use xlog_rs::log;
/// log::set_level(log::Level::Trace);
/// assert_eq!(log::level(), log::Level::Trace);
/// ```
pub fn set_level(level: Level) {
    unsafe {
        LEVEL = level;
    }
}
/// Returns the current log level.
pub fn level() -> Level {
    unsafe { LEVEL }
}
/// Logs a message at the trace level.
pub fn trace(s: &str) {
    unsafe {
        LOGGER.log(Level::Trace, s);
    }
}
/// Logs a message at the debug level.
pub fn debug(s: &str) {
    unsafe {
        LOGGER.log(Level::Debug, s);
    }
}
/// Logs a message at the info level.
pub fn info(s: &str) {
    unsafe {
        LOGGER.log(Level::Info, s);
    }
}
/// Logs a message at the warn level.
pub fn warn(s: &str) {
    unsafe {
        LOGGER.log(Level::Warn, s);
    }
}
/// Logs a message at the error level.
pub fn error(s: &str) {
    unsafe {
        LOGGER.log(Level::Error, s);
    }
}
