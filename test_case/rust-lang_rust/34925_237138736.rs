 rust
macro_rules! outer {
    ($b:tt $c:ident) => {
        macro_rules! inner {
            ($a $b $c) => { }
        }
    }
}
outer!(: ident); // NB: must be `:`
inner!(foo);
