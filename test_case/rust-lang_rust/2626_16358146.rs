
// Regression test for memory leaks
#[ignore(cfg(windows))] // FIXME (#2626)
pub fn test_leaks() {
    run::run_program("echo", []);
    run::start_program("echo", []);
    run::program_output("echo", []);
}
