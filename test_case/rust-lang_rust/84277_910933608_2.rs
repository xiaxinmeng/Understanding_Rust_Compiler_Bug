rust
macro_rules! e {
    ($($token:tt)+) => {
        MyOption::<_>::from($($token)+)?
    };
}
