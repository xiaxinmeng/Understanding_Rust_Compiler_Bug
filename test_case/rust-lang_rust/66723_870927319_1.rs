
error[E0283]: type annotations needed
  --> src/lib.rs:11:25
   |
11 |     Wrapper::First((22, Default::default()))
   |                         ^^^^^^^^^^^^^^^^ cannot infer type
   |
   = note: cannot satisfy `_: Default`
   = note: required by `std::default::Default::default`
