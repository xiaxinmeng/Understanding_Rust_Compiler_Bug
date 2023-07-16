
   Compiling pnetlink v0.1.0 (file:///home/robertc/personal/rust/rusty_rail/../pnetlink)
error[E0308]: mismatched types
  --> ../pnetlink/build.rs:26:31
   |
26 |         pnet_macros::register(&mut registry);
   |                               ^^^^^^^^^^^^^ expected struct `syntex::Registry`, found a different struct `syntex::Registry`
   |
   = note: expected type `&mut syntex::Registry`
   = note:    found type `&mut syntex::Registry`
note: Perhaps two different versions of crate `syntex` are being used?
  --> ../pnetlink/build.rs:26:31
   |
26 |         pnet_macros::register(&mut registry);
   |                               ^^^^^^^^^^^^^

