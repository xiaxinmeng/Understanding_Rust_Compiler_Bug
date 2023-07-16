rust
ExitCode::SUCCESS.exit_with()
// vs
ExitCode::SUCCESS.exit_process()

let code = ExitCode::FAILURE;
code.exit_with()
// vs
code.exit_process()
