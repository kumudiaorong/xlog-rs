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
    pub fn log(&mut self, args: std::fmt::Arguments<'_>) {
        if let Some(ref mut to) = self.to {
            let _ = to.write_fmt(args).unwrap();
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
#[inline]
pub fn set_level(level: Level) {
    unsafe {
        LEVEL = level;
    }
}
/// Returns the current log level.
#[inline]
pub fn level() -> Level {
    unsafe { LEVEL }
}
/// Logs a message at the specified level.
#[inline]
pub fn log(args: std::fmt::Arguments<'_>) {
    unsafe {
        LOGGER.log(args);
    }
}
/// Dispatch message with the type and level
#[macro_export]
macro_rules! log_dispatch {
    ($lv:literal $level:ident opt,$val:expr,$($tail:tt)*) => {
        {
            let mut temp = $val;
            if $crate::log::level() <= $crate::log::Level::$level && temp.is_none() {
                $crate::log::log(format_args!("[{}][OPT] {}\n",$lv,format_args!($($tail)*)));
            }
            temp
        }
    };
    ($lv:literal $level:ident res,$val:expr,$($tail:tt)*) => {
        ($val).map_err(|e|{
            if $crate::log::level() <= $crate::log::Level::$level{
                $crate::log::log(format_args!("[{}][RES] {} [E]:{}\n",$lv,format_args!($($tail)*),e));
            }
            e
        })
    };
    ($lv:literal $level:ident $($tail:tt)*) => {
        if $crate::log::level() <= $crate::log::Level::$level{
            $crate::log::log(format_args!("[{}] {}\n",$lv,format_args!($($tail)*)));
        }
    }
}
/// Logs a message at the trace level.
///
/// #Example
///
/// ```
/// use xlog_rs::log;
/// xlog_rs::trace!("{}", "abc");
/// let mut some = Some(());
/// let _ = xlog_rs::trace!(opt, some, "{}", "none");
/// some = None;
/// let _ = xlog_rs::trace!(opt, some, "{}", "none");
/// let mut ok = Ok(());
/// let _ = xlog_rs::trace!(res, ok, "{}", "error");
/// ok = Err("error");
/// let _ = xlog_rs::trace!(res, ok, "{}", "error");
/// ```
#[macro_export]
macro_rules! trace {
    ($($tail:tt)*) => {
        $crate::log_dispatch!("TRACE" Trace $($tail)*);
    };
}
/// Logs a message at the debug level.
#[macro_export]
macro_rules! debug {
    ($($tail:tt)*) => {
        $crate::log_dispatch!("DEBUG" Debug $($tail)*);
    };
}
/// Logs a message at the info level.
#[macro_export]
macro_rules! info {
    ($($tail:tt)*) => {
        $crate::log_dispatch!("INFO" Info $($tail)*);
    };
}
/// Logs a message at the warn level.
#[macro_export]
macro_rules! warn {
    ($($tail:tt)*) => {
        $crate::log_dispatch!("WARN" Warn $($tail)*);
    };
}
/// Logs a message at the error level.
#[macro_export]
macro_rules! error {
    ($($tail:tt)*) => {
        $crate::log_dispatch!("ERROR" Error $($tail)*);
    };
}
