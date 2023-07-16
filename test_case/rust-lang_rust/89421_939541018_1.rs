
error: unconstrained generic constant
 --> src/main.rs:5:17
  |
5 | type OnChainT = [u8;Self::len];
  |                 ^^^^^^^^^^^^^^
  |
  = help: try adding a `where` bound using this expression: `where [(); Self::len]:`
