rs
warning: the feature `generic_associated_types` is incomplete and may not be safe to use and/or cause compiler crashes
 --> src/main.rs:1:12
  |
1 | #![feature(generic_associated_types)]
  |            ^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(incomplete_features)]` on by default
  = note: see issue #44265 <https://github.com/rust-lang/rust/issues/44265> for more information

error[E0311]: the parameter type `S` may not live long enough
  --> src/main.rs:23:5
   |
22 | fn foo<'a, S: Trait + 'a>() {
   |            -- help: consider adding an explicit lifetime bound...: `S: 'b +`
23 |     future::<'a, S, _>(f::<'a, S>());
   |     ^^^^^^^^^^^^^^^^^^
   |
   = note: the parameter type `S` must be valid for any other region...
   = note: ...so that the type `S` will meet its required lifetime bounds

error: aborting due to previous error; 1 warning emitted
