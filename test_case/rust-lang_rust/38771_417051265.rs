
error[E0308]: mismatched types
  --> <assert_eq macros>:5:22
   |
1  | / ( $ left : expr , $ right : expr ) => (
2  | | {
3  | | match ( & $ left , & $ right ) {
4  | | ( left_val , right_val ) => {
5  | | if ! ( * left_val == * right_val ) {
   | |                      ^^^^^^^^^^^ expected u64, found usize
...  |
20 | |  right: `{:?}`: {}"# ,
21 | | left_val , right_val , format_args ! ( $ ( $ arg ) + ) ) } } } } ) ;
   | |____________________________________________________________________- in this expansion of `assert_eq!`
   |
  ::: test.rs:2:5
   |
2  |       assert_eq!(10u64, 10usize);
   |       --------------------------- in this macro invocation
