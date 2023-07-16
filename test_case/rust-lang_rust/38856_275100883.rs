rust
pub fn run_args(cmd: &str, args: &[String], shell: bool) -> Result<(Output)> {
    let output;

    if args.len() > 0 {
        if !shell {
            output = try!(Command::new(cmd).args(&args).output());
        } else {
    ...
