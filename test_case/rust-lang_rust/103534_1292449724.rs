rust
macro_rules! what {
    ($e:expr) => { compile_error!("no") };
    ($($tt:tt)*) => {};
}

what! {
    match 1;
}
