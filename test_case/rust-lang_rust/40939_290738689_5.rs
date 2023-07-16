rust
macro_rules! quote {
    () => { TokenStream::empty() };
    ($($t:tt)*) => { [ $( quote_tree!($t), )* ].iter().cloned().collect::<TokenStream>() };
}
