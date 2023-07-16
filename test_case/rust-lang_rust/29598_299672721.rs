rust
macro_rules! some_broken_macro {
    (...) => {
        struct $whatever { $($foo: $bar),* }
        // Let's say this causes a compile error for some reason, so macro expansion never finishes.
        // To debug, I could wrap the macro output in log_syntax!(...) to see what it would output.
    }
}
