rust
macro_rules! foo {
    ($dol:tt $a:ident) => {
        macro_rules! bar {
            ($dol$a:ident) => { $dol$a }
        }
    }
}
