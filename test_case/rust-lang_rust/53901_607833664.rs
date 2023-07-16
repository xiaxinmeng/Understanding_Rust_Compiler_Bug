rust
error: attribute must be of the form `#[link(name = "...", /*opt*/ kind = "dylib|static|...", /*opt*/ cfg = "...")]`
 --> src/main.rs:1:1
  |
1 | #[link="reiojregioergioregiojrge"]
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[deny(ill_formed_attribute_input)]` on by default
  = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
  = note: for more information, see issue #57571 <https://github.com/rust-lang/rust/issues/57571>

error: aborting due to previous error
