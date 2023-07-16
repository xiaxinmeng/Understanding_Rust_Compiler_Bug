
error[E0382]: borrow of moved value: `maybe_s`
  --> src/playground/while_let_binding_move.rs:20:26
   |
12 |         let mut maybe_s = Some(SomeStruct { val: 0 });
   |             ----------- move occurs because `maybe_s` has type `Option<while_let_binding_move::SomeStruct>`, which does not implement the `Copy` trait
13 |         let mut maybe_t = Some(SomeStruct { val: 1 });
14 |         if let (Some(s), Some(t)) = (maybe_s, maybe_t) {
   |                                      ------- value moved here
...
17 |             maybe_s = Some(s);
   |             ----------------- this reinitialization might get skipped
...
20 |         println!("{:?}", maybe_s);
   |                          ^^^^^^^ value borrowed here after move
   |
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0382]: borrow of moved value: `maybe_t`
  --> src/playground/while_let_binding_move.rs:21:26
   |
13 |         let mut maybe_t = Some(SomeStruct { val: 1 });
   |             ----------- move occurs because `maybe_t` has type `Option<while_let_binding_move::SomeStruct>`, which does not implement the `Copy` trait
14 |         if let (Some(s), Some(t)) = (maybe_s, maybe_t) {
   |                                               ------- value moved here
...
18 |             maybe_t = Some(t);
   |             ----------------- this reinitialization might get skipped
...
21 |         println!("{:?}", maybe_t);
   |                          ^^^^^^^ value borrowed here after move
   |
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
