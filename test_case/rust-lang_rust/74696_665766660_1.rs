
error: this operation will panic at runtime
 --> src/main.rs:3:21
  |
3 |         let _val = &(1/0);
  |                     ^^^^^ attempt to divide 1_i32 by zero
  |
  = note: `#[deny(unconditional_panic)]` on by default

error: reaching this expression at runtime will panic or abort
 --> src/main.rs:3:21
  |
3 |         let _val = &(1/0);
  |                    -^^^^^
  |                     |
  |                     dividing by zero
  |
  = note: `#[deny(const_err)]` on by default

error: erroneous constant used
 --> src/main.rs:3:20
  |
3 |         let _val = &(1/0);
  |                    ^^^^^^ referenced constant has errors
