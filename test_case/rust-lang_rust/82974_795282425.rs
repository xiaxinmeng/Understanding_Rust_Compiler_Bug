rust
match signum {
    libc::SIGSEGV => "Segmentation fault (SIGSEGV)",
    libc::SIGKILL => "Killed (SIGKILL)",
    ...
}
