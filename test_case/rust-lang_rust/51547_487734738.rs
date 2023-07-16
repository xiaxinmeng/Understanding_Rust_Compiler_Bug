
error: unexpected `self` argument in function
 --> src/lib.rs:1:14
  |
1 | fn foo(self: i32) -> i32 { self }
  |              ^^^ `self` is only valid as the first argument of an associated function

error[E0424]: expected value, found module `self`
 --> src/lib.rs:1:28
  |
1 | fn foo(self: i32) -> i32 { self }
  |                            ^^^^ `self` value is a keyword only available in methods with `self` parameter
