use std::fmt::Display;

struct Logger {
    to: Option<Box<dyn std::io::Write>>,
}
static mut LOGGER: Logger = Logger { to: None };
#[derive(PartialEq, PartialOrd, Copy, Clone)]
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
    pub fn log(&mut self, level: Level, s: impl Display) {
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
/// Initializes the logger with a filename.
pub fn with_file(name: &str) {
    // Box::new(to);
    unsafe {
        LOGGER = Logger {
            to: Some(Box::new(
                std::fs::File::options()
                    .create(true)
                    .append(true)
                    .open(name.to_string())
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
/// Logs a message at the specified level.
///
/// #Example
///
/// ```
/// use xlog_rs::log;
/// log::init(std::io::stdout(), log::Level::Trace);
/// log::log(log::Level::Debug,"abc");
/// ```
pub fn log(level: Level, msg: impl Display) {
    unsafe {
        LOGGER.log(level, msg);
    }
}
///
pub fn log_opt<T, F>(level: Level, value: Option<T>, map: F, msg: impl Display)
where
    F: Fn(T),
{
    match value {
        Some(value) => map(value),
        None => log(level, msg),
    }
}
///
pub fn log_res<T, E, F>(level: Level, value: Result<T, E>, map: F)
where
    E: Display,
    F: Fn(T),
{
    match value {
        Ok(value) => map(value),
        Err(e) => log(level, e.to_string()),
    }
}
/// Dispatch message with the type and level
#[macro_export]
macro_rules! log_dispatch {
    ($level:ident $msg:expr) => {
        $crate::log::log($crate::log::Level::$level, $msg);
    };
    ($level:ident opt,$val:expr,$map:expr,$msg:expr) => {
        $crate::log::log_opt($crate::log::Level::$level, $val, $map, $msg);
    };
    ($level:ident res,$val:expr,$map:expr) => {
        $crate::log::log_res($crate::log::Level::$level, $val, $map);
    };
}
/// Logs a message at the trace level.
///
/// #Example
///
/// ```
/// use xlog_rs::log;
/// log::init(std::io::stdout(), log::Level::Trace);
/// xlog_rs::trace!("abc");
/// let mut some = Some(());
/// xlog_rs::trace!(opt, some, |_| { xlog_rs::trace!("some") }, "none");
/// some = None;
/// xlog_rs::trace!(opt, some, |_| { xlog_rs::trace!("some") }, "none");
/// let mut ok = Ok(());
/// xlog_rs::trace!(res, ok, |_| { xlog_rs::trace!("ok") });
/// ok = Err("error");
/// xlog_rs::trace!(res, ok, |_| { xlog_rs::trace!("opt") });
/// ```
#[macro_export]
macro_rules! trace {
    ($($tail:tt)*) => {
        $crate::log_dispatch!(Trace $($tail)*);
    };
}
/// Logs a message at the debug level.
#[macro_export]
macro_rules! debug {
    ($($tail:tt)*) => {
        $crate::log_dispatch!(Debug $($tail)*);
    };
}
/// Logs a message at the info level.
#[macro_export]
macro_rules! info {
    ($($tail:tt)*) => {
        $crate::log_dispatch!(Info $($tail)*);
    };
}
/// Logs a message at the warn level.
#[macro_export]
macro_rules! warn {
    ($($tail:tt)*) => {
        $crate::log_dispatch!(Warn $($tail)*);
    };
}
/// Logs a message at the error level.
#[macro_export]
macro_rules! error {
    ($($tail:tt)*) => {
        $crate::log_dispatch!(Error $($tail)*);
    };
}
