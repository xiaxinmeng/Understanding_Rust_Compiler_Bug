rust
    #[unstable(feature = "process_sigmask", issue = "none")]
    fn block_signal(&mut self, sig: std::os::unix::signal::Signal) -> &mut process::Command;
    #[unstable(feature = "process_sigmask", issue = "none")]
    fn unblock_signal(&mut self, sig: std::os::unix::signal::Signal) -> &mut process::Command;
    