use xlog_rs::{log, trace};
mod common;
#[test]
fn log_test() {
    common::init();
    log::log(log::Level::Debug, "abc");
    common::assert(&b"[DEBUG] abc\n".to_vec());
}
#[test]
fn trace_test() {
    common::init();
    trace!("abc");
    common::assert(&b"[TRACE] abc\n".to_vec());
    let mut some = Some(());
    trace!(opt, some, |_| { trace!("some") }, "none");
    common::assert(&b"[TRACE] abc\n[TRACE] some\n".to_vec());
    some = None;
    trace!(opt, some, |_| { trace!("some") }, "none");
    common::assert(&b"[TRACE] abc\n[TRACE] some\n[TRACE] none\n".to_vec());
    let mut ok = Ok(());
    trace!(res, ok, |_| { trace!("ok") });
    common::assert(&b"[TRACE] abc\n[TRACE] some\n[TRACE] none\n[TRACE] ok\n".to_vec());
    ok = Err("error");
    trace!(res, ok, |_| { trace!("opt") });
    common::assert(
        &b"[TRACE] abc\n[TRACE] some\n[TRACE] none\n[TRACE] ok\n[TRACE] error\n".to_vec(),
    );
}
