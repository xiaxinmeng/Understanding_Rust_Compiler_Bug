
warning: function is never used: `use_c`
 --> src/lib.rs:7:4
  |
7 | fn use_c<B, C: TraitC<B>>(mut a: C::Thing, b: B) {
  |    ^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: 1 warning emitted
