rust
macro_rules! check {
    ($Z:ty) => { compile_error!("triggered"); };
    ($X:ty | $Y:ty) => { };
}

check! { i32 | u8 } // passes on stable & nightly
