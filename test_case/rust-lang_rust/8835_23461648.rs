
pub fn process_status<S: Str>(prog: &str, args: &[S]) -> int {
    let mut prog = Process::new(prog, args, ProcessOptions {
        env: None,
        dir: None,
        in_fd: Some(0),
        out_fd: Some(1),
        err_fd: Some(2)
    }).unwrap();
    prog.finish()
}
