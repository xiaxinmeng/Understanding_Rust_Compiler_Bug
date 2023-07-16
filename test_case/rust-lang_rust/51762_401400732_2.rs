rust
macro more_opaque_macro_def() { // x
    macro_rules! less_opaque_macro_call { // y
        () => { my_token }
    }
    less_opaque_macro_call!();
}

// now produces `my_token`, `my_token` has context `root -> x -> y`
less_opaque_macro_call!();
