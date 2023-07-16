rust
// librustc_driver/lib.rs
...
/// Get a list of extra command-line flags provided by the user, as strings.
///
/// This function is used during ICEs to show more information useful for
/// debugging, since some ICEs only happens with non-default compiler flags
/// (and the users don't always report them).
fn extra_compiler_flags() -> Option<(Vec<String>, bool)> {
    let mut args = Vec::new();
    for arg in env::args_os() {
        args.push(arg.to_string_lossy().to_string());
    }

    let matches = if let Some(matches) = handle_options(&args) { ...
