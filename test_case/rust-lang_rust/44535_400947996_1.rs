rust
macro_rules! give_me_foo_or_bar {
    (foo) => {};
    (bar) => {};
    ($x:ident) => {
        compile_error!("This macro only accepts `foo` or `bar`. Find location of error in next error");
        force_mismatch_on_error_token! ($x);
    }
}
