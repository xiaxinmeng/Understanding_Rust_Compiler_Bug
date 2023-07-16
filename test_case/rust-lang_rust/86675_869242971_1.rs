rust
macro_rules! normal_macro {
    ($name:ident!()) => {
        let _ = move |a: String| $name!();
    };
}
