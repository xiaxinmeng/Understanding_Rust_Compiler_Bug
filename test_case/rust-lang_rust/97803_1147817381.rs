rust
/// panic_any with this to std::process::exit but actually do cleanup
/// (also panic::set_hook to similarly match on this payload to silence printing)
struct ExitPanic(i32);

fn main() -> Result<(), eyre::Report> {
    // Wrap main up in a catch_panic so that we can use it to implement std::process::exit with
    // unwinding, allowing us to silently exit the program while still cleaning up.
    let result = std::panic::catch_unwind(real_main);
    match result {
        Ok(main_result) => main_result,
        Err(e) => {
            if let Some(ExitPanic(code)) = e.downcast_ref::<ExitPanic>() {
                // Exit panic, just silently exit with this status
                std::process::exit(*code);
            } else {
                // Normal panic, let it ride
                std::panic::resume_unwind(e);
            }
        }
    }
}
