rust
const FYI: () = {
    if !cfg!(debug_assertions) {
        dbgprintln!("This code is terrible and probably shouldn't be run in production.");
    }
};
