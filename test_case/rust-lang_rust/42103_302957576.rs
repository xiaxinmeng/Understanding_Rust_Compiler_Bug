console
note: trace_macro
 --> main.rs:5:5
  |
5 |     assert_eq!(vec![1, 2, 3,].len(), 3);
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: expanding `assert_eq! { vec ! [ 1 , 2 , 3 , ] . len (  ) , 3 }`
  = note: to `{
          match ( & vec!(1 , 2 , 3 ,).len() , & 3 ) {
          ( left_val , right_val ) => {
          if ! ( * left_val == * right_val ) {
          panic ! (
          "assertion failed: `(left == right)` \
                                     (left: `{:?}`, right: `{:?}`)"
          , left_val , right_val ) } } } }`
  = note: expanding `panic! { "assertion failed: `(left == right)` \
                                     (left: `{:?}`, right: `{:?}`)"
          , left_val , right_val }`
  = note: to `{
          $crate :: rt :: begin_panic_fmt (
          & format_args ! (
          "assertion failed: `(left == right)` (left: `{:?}`, right: `{:?}`)" , left_val
          , right_val ) , {
          static _FILE_LINE : ( & 'static str , u32 ) = ( file ! (  ) , line ! (  ) ) ;
          & _FILE_LINE } ) }`
