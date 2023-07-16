rust
macro_rules! foo {
    ($n:tt) => {
        // The lint does not recognize the possible nested macro definition.
        $n! bar {
            // $x is free unless $n is `macro_rules`
            ($x:tt) => { $x };
        }
    };
}
