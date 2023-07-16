
warning: a closure's body is not determined by its enclosing block
  --> $DIR/block-enclosed-closure.rs:14:33
   |
14 |     let _p = Some(45).and_then({|x|
   |                                 ^^^ this closure's body is not determined by its enclosing block
   |
note: this is the closure's block...
  --> $DIR/block-enclosed-closure.rs:14:33
   |
14 |       let _p = Some(45).and_then({|x|
   |  _________________________________^
17 | |         println!("hi");
   | |______________________^
note: ...while this enclosing block...
  --> $DIR/block-enclosed-closure.rs:14:32
   |
14 |       let _p = Some(45).and_then({|x|
   |  ________________________________^
17 | |         println!("hi");
18 | |         Some(x * 2)
19 | |     });
   | |_____^
note: ...implicitely returns
  --> $DIR/block-enclosed-closure.rs:18:9
   |
18 |         Some(x * 2)
   |         ^^^^^^^^^^^
help: you should open the block *after* the closure's argument list
   |
14 |     let _p = Some(45).and_then(|x| {
   |                                ^^^^^

error[E0425]: cannot find value `x` in this scope
  --> $DIR/block-enclosed-closure.rs:18:14
   |
18 |         Some(x * 2)
   |              ^ not found in this scope

error[E0277]: the trait bound `std::option::Option<_>: std::ops::FnOnce<({integer},)>` is not satisfied
  --> $DIR/block-enclosed-closure.rs:14:23
   |
14 |     let _p = Some(45).and_then({|x|
   |                       ^^^^^^^^ the trait `std::ops::FnOnce<({integer},)>` is not implemented for `std::option::Option<_>`

error: aborting due to 2 previous errors
