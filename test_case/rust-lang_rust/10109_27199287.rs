 rust
fn main() {
    let args = os::args();
    if args.len() >= 2 && args[1] == "signal" {
        // called like `./testname signal`, so raise a signal...
    } else {
        // work out the path to this executable.
        let mut self_path = os::self_exe_path().unwrap();
        self_path.push(args[0]);
        let status = run::process_status(format!("{}", self_path.display()), 
                                         [~"signal"]);
        assert_eq!(status, ExitSignal(/* whatever */));
    }
}
