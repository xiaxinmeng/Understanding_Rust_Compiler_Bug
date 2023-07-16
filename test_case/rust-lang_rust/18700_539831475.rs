rust
macro_rules! strip_plus {
    {+ $($rest:tt)* } => { $($rest)* }
}
