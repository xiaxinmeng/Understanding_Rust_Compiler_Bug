rust
macro_rules! less_opaque_macro_call { // y
    () => { my_token }
}

macro more_opaque_macro_def() { // x
    less_opaque_macro_call!();
}

...
// would produce `my_token`, `my_token` would have context `root -> y`, BUT...
less_opaque_macro_call!();
