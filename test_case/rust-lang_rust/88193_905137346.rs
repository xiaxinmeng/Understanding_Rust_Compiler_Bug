rust
#[cfg(test)]
fn test_object_safety() {
    #[cfg(windows)]
    use std::os::windows::process::CommandExt;
    #[cfg(unix)]
    use std::os::unix::process::CommandExt;
    fn takes_dyn_command_ext(_: &dyn CommandExt) {}
    takes_dyn_command_ext(&std::process::Command::new(""));
}
