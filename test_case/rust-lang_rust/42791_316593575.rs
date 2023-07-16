rust
#[cfg(windows)]
mod execs {
    use std::process::Command;
    pub fn npm() -> Command {
        let mut cmd = Command::new("cmd");
        cmd.arg("/c").arg("npm.cmd");
        cmd
    }
    pub fn stylus() -> Command {
        let mut cmd = Command::new("cmd");
        cmd.arg("/c").arg("stylus.cmd");
        cmd
    }
}
#[cfg(not(windows))]
mod execs {
    use std::process::Command;
    pub fn npm() -> Command {
        Command::new("npm")
    }
    pub fn stylus() -> Command {
        Command::new("stylus")
    }
}
