
  pub fn spawn_process(prog: &str, args: &[~str],
                       env: &Option<~[(~str,~str)]>,
                       dir: &Option<~str>,
                       in_fd: c_int, out_fd: c_int, err_fd: c_int) -> pid_t { ... }
  pub fn run_program(prog: &str, args: &[~str]) -> int { ... }
  pub fn start_program(prog: &str, args: &[~str]) -> Program { ... }
  pub fn program_output(prog: &str, args: &[~str]) -> ProgramOutput { ... }
  