 rust
// the z flows into and out of two macros (g & f) along one path, and one (just g) along the
// other, so the result of the whole thing should be "let z_123 = 3; z_123
macro_rules! g (
    ($x:ident) =>
    (
        {macro_rules! f(
            ($y:ident)=>({let $y=3;$x})
        );
         f!($x)})
);

fn a(){g!(z)}
