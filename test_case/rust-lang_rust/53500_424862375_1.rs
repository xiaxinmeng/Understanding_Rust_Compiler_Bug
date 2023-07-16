
error[E0308]: if and else have incompatible types
 --> src/main.rs:9:20
  |
9 |     let do_stuff = if true { manipulation } else { identity };
  |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ incorrect number of function parameters
  |
  = note: expected type `fn(u32, u32) -> u32 {manipulation}`
             found type `fn(_) -> _ {std::convert::identity::<_>}`
