rust
macro_rules! foo {
    () => {
        // The lint does not look at the definition of `def`, thus does not recognize
        // the nested macro definition, and thus reports $x as free.
        def!({ ($x:tt) => { $x }; });
    };
}
macro_rules! def { ($body:tt) => { macro_rules! bar $body }; }
