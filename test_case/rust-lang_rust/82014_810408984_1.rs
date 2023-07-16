
39 | |         ///
   | |___________^
   | 
   | 
  ::: library/core/src/num/shells/i128.rs:13:1
   |
13 |   int_module! { i128, #[stable(feature = "i128", since="1.26.0")] }
   |
   |
   = note: the link appears in this line:
           
           The largest value that can be represented by this integer type. Use [`i128::MAX`] instead.
                                                                                ^^^^^^^^^^^
   = note: `i128::MAX` may be in a private module with all re-exports marked as `#[doc(no_inline)]`


error: documentation for `MIN` links to item `i16::MIN` which will not have documentation generated
   |
   |
6  | /         #[doc = concat!(
7  | |             "The smallest value that can be represented by this integer type. Use ",
8  | |             "[`", stringify!($T), "::MIN", "`] instead."
9  | |         )]
19 | |         /// 