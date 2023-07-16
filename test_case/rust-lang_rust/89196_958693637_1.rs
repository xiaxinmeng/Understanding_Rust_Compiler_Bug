
error[E0277]: the trait bound `for<'a> <&'a _ as IntoIterator>::IntoIter: Clone` is not satisfied
  --> src/main.rs:44:7
   |
44 |     S.e(ST, arr);
   |       ^ the trait `for<'a> Clone` is not implemented for `<&'a _ as IntoIterator>::IntoIter`
   |
   = help: the following implementations were found:
             <&T as Clone>
note: required because of the requirements on the impl of `Tr` for `E<_, _>`
  --> src/main.rs:29:12
   |
29 | impl<A, B> Tr for E<A, B>
   |            ^^     ^^^^^^^
note: required because of the requirements on the impl of `R<E<_, _>>` for `S`
  --> src/main.rs:16:9
   |
16 | impl<A> R<A> for S where A: Tr {}
   |         ^^^^     ^

For more information about this error, try `rustc --explain E0277`.
error: could not compile `playground` due to previous error

