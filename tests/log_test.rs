use xlog_rs::{log, trace};
mod common;
#[test]
fn log_test() {
    common::init();
    log::log(format_args!("abc"));
    common::assert("abc");
}
#[test]
fn simple_test() {
    common::init();
    trace!("{}", "abc");
    common::assert("[TRACE] abc\n");
}
#[test]
fn opt_test() {
    common::init();
    let mut some = Some(());
    let _ = trace!(opt, some, "{}", "none");
    common::assert("");
    some = None;
    let _ = trace!(opt, some, "{}", "none");
    common::assert("[TRACE][OPT] none\n");
}
#[test]
fn res_test() {
    common::init();
    let mut ok = Ok(());
    let _ = trace!(res, ok, "{}", "error");
    common::assert("");
    ok = Err("error");
    let _ = trace!(res, ok, "{}", "error");
    common::assert("[TRACE][RES] error [E]:error\n");
}
