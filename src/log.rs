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
pub fn init(to: impl std::io::Write + 'static) {
    // Box::new(to);
    unsafe {
        LOGGER = Logger {
            to: Some(Box::new(to)),
        };
    }
}
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
pub fn set_level(level: Level) {
    unsafe {
        LEVEL = level;
    }
}
pub fn trace(s: &str) {
    unsafe {
        LOGGER.log(Level::Trace, s);
    }
}
pub fn debug(s: &str) {
    unsafe {
        LOGGER.log(Level::Debug, s);
    }
}
pub fn info(s: &str) {
    unsafe {
        LOGGER.log(Level::Info, s);
    }
}
pub fn warn(s: &str) {
    unsafe {
        LOGGER.log(Level::Warn, s);
    }
}
pub fn error(s: &str) {
    unsafe {
        LOGGER.log(Level::Error, s);
    }
}
