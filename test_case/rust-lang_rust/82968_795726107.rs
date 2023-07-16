
warning: unnecessary parentheses around assigned value
   --> src/gate.rs:144:19
    |
144 |     let c = test!((-a));
    |                   ^^^^ help: remove these parentheses
    |
    = note: `#[warn(unused_parens)]` on by default
