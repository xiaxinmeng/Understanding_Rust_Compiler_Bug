
warning: field is never used: `x`
  --> test.rs:11:10
   |
6  | |             $($name : $field),*
   | |____________________________^
...
11 |   state! { x: i64 }
   |  _---------^-------
   | | |
   | | in this macro invocation
   |
   = note: #[warn(dead_code)] on by default


