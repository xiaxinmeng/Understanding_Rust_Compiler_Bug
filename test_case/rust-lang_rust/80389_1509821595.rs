
error: impl method assumes more implied bounds than the corresponding trait method
 --> src/main.rs:9:22
  |
9 |     fn f<'b>(mut _x: &'static &'b Ty, y: &'b Ty) -> &'static Ty {
  |                      ^^^^^^^^^^^^^^^ help: replace this type to make the impl signature compatible: `&'static &'a Box<&'static u8>`
  |
  = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
  = note: for more information, [see issue #105572 <https://github.com/rust-lang/rust/issues/105572>](https://github.com/rust-lang/rust/issues/105572)
  = note: `#[deny(implied_bounds_entailment)]` on by default
  