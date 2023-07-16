rust
error[E0282]: type annotations needed
 --> tests/ui/issues/issue-16966.rs:2:12
  |
2 |     panic!(std::default::Default::default());
  |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot infer type for type parameter `M` declared on the function `begin_panic`
