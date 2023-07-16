rust
macro_rules! normal_macro {
    ($name:tt $exclam:tt $parens:tt) => {
        let _ = move |a: String| $name $exclam ();
    };
}
