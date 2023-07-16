
39 | |         ///
   | |___________^
   | 
   | 
  ::: library/core/src/num/shells/i32.rs:13:1
   |
13 |   int_module! { i32 }
   |
   |
   = note: the link appears in this line:
           
           The largest value that can be represented by this integer type. Use [`i32::MAX`] instead.
                                                                                ^^^^^^^^^^
   = note: `i32::MAX` may be in a private module with all re-exports marked as `#[doc(no_inline)]`


error: documentation for `MIN` links to item `i64::MIN` which will not have documentation generated
   |
   |
6  | /         #[doc = concat!(
7  | |             "The smallest value that can be represented by this integer type. Use ",
8  | |             "[`", stringify!($T), "::MIN", "`] instead."
9  | |         )]
19 | |         /// 