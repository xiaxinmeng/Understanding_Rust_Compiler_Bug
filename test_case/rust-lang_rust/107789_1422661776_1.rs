rust
error[E0282]: type annotations needed
  --> $DIR/issue-16966.rs:2:5
   |
LL |     panic!(std::default::Default::default());
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot infer type of the type parameter `M` declared on the function `begin_panic`
   |
   = note: this error originates in the macro `$crate::panic::panic_2015` which comes from the expansion of the macro `panic` (in Nightly builds, run with -Z macro-backtrace for more info)
