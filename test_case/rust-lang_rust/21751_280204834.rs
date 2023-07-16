
rustc 1.15.1 (021bd294c 2017-02-08)
warning: unreachable statement, #[warn(unreachable_code)] on by default
 --> <anon>:3:28
  |
3 |         while { break; } { println!("never"); }
  |                            ^^^^^^^^^^^^^^^^^^
  |
  = note: this error originates in a macro outside of the current crate

warning: unreachable statement, #[warn(unreachable_code)] on by default
 --> <anon>:4:9
  |
4 |         println!("?");
  |         ^^^^^^^^^^^^^^
  |
  = note: this error originates in a macro outside of the current crate

?
?
?
?
?

Program ended.
