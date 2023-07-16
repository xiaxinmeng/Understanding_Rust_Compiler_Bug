
macro_rules! m {
    ($tt:tt #) => ()
}

m!(r#"abc"##); // OK
