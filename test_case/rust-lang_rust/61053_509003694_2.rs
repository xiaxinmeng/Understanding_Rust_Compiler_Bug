rust
macro_rules! foo {
    () => {
        // The lint thinks bar is a nested macro definition but it's not.
        stringify!(macro_rules! bar { () => { $x }; })
    };
}
