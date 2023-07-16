
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: error: the type `T` has an unknown layout
                                 while computing layout for type T
 [--> src/main.rs:6:5
](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021#)  |
6 |     fn foo(self: T) -> dyn Trait<T>;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: delayed at compiler/rustc_trait_selection/src/traits/object_safety.rs:470:34

error: internal compiler error: receiver when `Self = ()` should have a Scalar ABI; found None
 [--> src/main.rs:6:5
](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021#)  |
6 |     fn foo(self: T) -> dyn Trait<T>;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: delayed at compiler/rustc_trait_selection/src/traits/object_safety.rs:486:30

error: internal compiler error: receiver when `Self = (dyn Trait<T> + 'static)` should have a ScalarPair ABI; found None
 [--> src/main.rs:6:5
](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021#)  |
6 |     fn foo(self: T) -> dyn Trait<T>;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: delayed at compiler/rustc_trait_selection/src/traits/object_safety.rs:506:30
