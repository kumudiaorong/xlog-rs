use std::io::Cursor;
use xlog::log;
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
pub fn assert(except: &str) {
    assert_eq!(
        unsafe {
            String::from_utf8(LOG_BUF.as_mut().unwrap().clone())
                .unwrap()
                .as_str()
        },
        except
    );
}
