
#[test] #[should_fail]
pub fn test() {
    let mut res = None;
    do task::task().future_result(|+r| res = Some(move r)).spawn {
                die!(~"woops");
    }
    unsafe { libc::sleep(1); }
    let res = option::swap_unwrap(&mut res);
    res.recv();
}
