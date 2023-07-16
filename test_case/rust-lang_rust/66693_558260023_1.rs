
error[E0308]: mismatched types
 --> src/lib.rs:2:28
  |
2 | const _: () = core::panic!(1234);
  |                            ^^^^ expected `&str`, found integer
