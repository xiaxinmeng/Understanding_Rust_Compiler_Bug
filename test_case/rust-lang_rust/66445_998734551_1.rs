
error[E0277]: the trait bound `Vec<u8>: Extend<{float}>` is not satisfied
  --> src/main.rs:4:5
   |
4  |     once(async { 0.0 }).collect().await
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Extend<{float}>` is not implemented for `Vec<u8>`
   |
   = help: the following implementations were found:
             <Vec<T, A> as Extend<&'a T>>
             <Vec<T, A> as Extend<T>>
   = note: required because of the requirements on the impl of `futures::Future` for `Collect<futures::stream::Once<impl futures::Future>, Vec<u8>>`
note: required by `futures::Future::poll`

error[E0277]: the trait bound `Vec<u8>: Extend<{float}>` is not satisfied
 --> src/main.rs:4:25
  |
4 |     once(async { 0.0 }).collect().await
  |                         ^^^^^^^ the trait `Extend<{float}>` is not implemented for `Vec<u8>`
  |
  = help: the following implementations were found:
            <Vec<T, A> as Extend<&'a T>>
            <Vec<T, A> as Extend<T>>

For more information about this error, try `rustc --explain E0277`.
error: could not compile `playground` due to 2 previous errors
