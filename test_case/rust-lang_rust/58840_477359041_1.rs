
error[E0277]: expected a `std::ops::Fn<(_,)>` closure, found `impl std::ops::Fn<(((_, _), _),)>`
  --> src/main.rs:32:12
   |
32 |   let t8 = pair_panic(t7, o_panic(f, g));
   |            ^^^^^^^^^^ expected an `Fn<(_,)>` closure, found `impl std::ops::Fn<(((_, _), _),)>`
   |
   = help: the trait `std::ops::Fn<(_,)>` is not implemented for `impl std::ops::Fn<(((_, _), _),)>`
note: required by `pair_panic`
  --> src/main.rs:9:1
   |
9  | / fn pair_panic<A,B,C>( f:impl Fn(A) -> B, g:impl Fn(A) -> C ) -> impl Fn(A) -> (B,C)
10 | |   where
11 | |     A: Copy
12 | | {
...  |
18 | |   }
19 | | }
   | |_^

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
error: Could not compile `panic`.

To learn more, run the command again with --verbose.
