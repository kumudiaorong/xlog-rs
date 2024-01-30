use std::io::Cursor;
use xlog_rs::log;
static mut LOG_BUF: Option<Vec<u8>> = None;
pub fn init() {
    unsafe {
        LOG_BUF = Some(vec![]);
    }
    log::init(
        unsafe { Cursor::new(LOG_BUF.as_mut().unwrap()) },
        log::Level::Trace,
    );
}
pub fn assert(except: &Vec<u8>) {
    assert_eq!(unsafe { LOG_BUF.as_mut().unwrap() }, except);
}
