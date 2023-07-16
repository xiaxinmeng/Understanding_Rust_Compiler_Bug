rust
macro_rules! normal_macro {
    ($name:ident $exclam:tt $parens:tt) => {
        let _ = move |a: String| $name $exclam $parens;
    };
}
