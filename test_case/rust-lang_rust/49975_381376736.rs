
   Compiling feature-in-mod v0.1.0 (file:///home/lampam/cpp/throwaway/feature/feature-in-mod)
error[E0658]: box pattern syntax is experimental (see issue #29641)
 --> src/transform.rs:7:18
  |
7 |         AST::Add(box AST::Add(ll, lr), r) => AST::Add(ll, Box::new(AST::Add(lr, r))),
  |                  ^^^^^^^^^^^^^^^^^^^^
  |
  = help: add #![feature(box_patterns)] to the crate attributes to enable

error: aborting due to previous error
