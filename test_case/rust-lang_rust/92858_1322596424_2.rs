
error[E0382]: borrow of moved value: `maybe_s`
  --> src/playground/while_let_binding_move.rs:16:26
   |
12 |         let mut maybe_s = Some(SomeStruct { val: 0 });
   |             ----------- move occurs because `maybe_s` has type `Option<while_let_binding_move::SomeStruct>`, which does not implement the `Copy` trait
13 |         if let (Some(s),) = (maybe_s,) {
   |                              ------- value moved here
14 |             maybe_s = Some(s);
   |             ----------------- this reinitialization might get skipped
15 |         }
16 |         println!("{:?}", maybe_s);
   |                          ^^^^^^^ value borrowed here after move
   |
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
