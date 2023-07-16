rust
macro_rules! what {
    ($e:expr) => { compile_error!("no") };
    ($($tt:tt)*) => {};
}

fn wh() {
    what! {
        match 1;
    }
}
