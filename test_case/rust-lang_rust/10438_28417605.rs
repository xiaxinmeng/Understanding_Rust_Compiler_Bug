
std::os::set_exit_status(match exit {
    std::io::process::ExitStatus(s) => s,
    _ => 1
});
