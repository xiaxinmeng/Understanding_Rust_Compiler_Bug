rust
fn main() -> Result<ExitCode, eyre::Report> {
    // Wrap main up in a catch_panic so that we can use it to implement std::process::exit with
    // unwinding, allowing us to silently exit the program while still cleaning up.
    let result = std::panic::catch_unwind(real_main);
    match result {
        Ok(main_result) => main_result,
        Err(e) => {
            if let Some(code) = e.downcast_ref::<ExitCode>() {
                // Exit panic, just silently exit with this status
                Ok(code)
            } else {
                // Normal panic, let it ride
                std::panic::resume_unwind(e);
            }
        }
    }
}
