 rust
// other, so the result of the whole thing should be "let z_123 = 3; z_123
macro_rules! g (
    ($x:ident) =>
    (
        {macro_rules! f(
            ($y:ident)=>({let $y=3;$x})
        );
         f!($x)})
)

fn a(){g!(z)}
