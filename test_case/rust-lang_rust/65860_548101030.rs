rust
macro_rules! delay_parsing { ($($tt:tt)*) => ($($tt)*) }

#[cfg(nightly)]
delay_parsing! {
    macro foo() {}
}
