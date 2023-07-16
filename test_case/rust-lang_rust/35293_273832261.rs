rust
mod process {
    fn propagate_exit(status: ExitStatus) -> ! {
        kill_all_other_threads();
        run_atexit_handlers();
        if let Ok(sig) = status.signal() {
            unmask_and_reset_handler_for_signal(sig);
            kill(getpid(), sig);
        } else {
            _exit(status.code().unwrap());
        }
        unreachable!();
    }
}
