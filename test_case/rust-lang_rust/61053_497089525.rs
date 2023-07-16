
macro_rules! outer {
    ($x:expr; $fragment:ident) => {
        macro_rules! inner { ($y:$fragment) => { $x + $y } }
    }
}
