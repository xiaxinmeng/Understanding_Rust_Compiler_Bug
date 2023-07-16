rust
macro_rules! typed_closure {
    (($($bound:tt)*), $closure:expr) => { {
        fn _typed_closure_id<F>(f: F) -> F where F: $($bound)* { f }
        _typed_closure_id($closure)
    } }
}

foo(typed_closure!((Fn(&u8) -> &u8), |x| x));
