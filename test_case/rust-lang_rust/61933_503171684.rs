
$ rustc +nightly file3.rs -Zexternal-macro-backtrace
error: unexpected `(` after qualified path
 --> <::alloc::macros::vec macros>:3:23
  |
1 | / ( $ elem : expr ; $ n : expr ) => (
2 | | $ crate :: vec :: from_elem ( $ elem , $ n ) ) ; ( $ ( $ x : expr ) , * ) => (
3 | | < [ _ ] > :: into_vec ( box [ $ ( $ x ) , * ] ) ) ; ( $ ( $ x : expr , ) * )
  | |                       ^ unexpected `(` after qualified path
4 | | => ( $ crate :: vec ! [ $ ( $ x ) , * ] )
  | |_________________________________________- in this expansion of `vec!`
  |
 ::: file3.rs:3:14
  |
3 |           Some(vec![x]) => (),
  |                -------
  |                |
  |                in this macro invocation
  |                in this macro invocation

error: aborting due to previous error
