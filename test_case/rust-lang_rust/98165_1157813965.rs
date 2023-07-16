plain
   Doc-tests core

running 3853 tests
...................................i..i......iiiiii..................................... 88/3853
.....................................................................FFF.....F.FFFFF.... 176/3853
.....................ii..............................iii.......i........................ 352/3853
........................................................................................ 440/3853
.............................i.......................................................... 528/3853
........................................................................................ 616/3853
---
error: aborting due to previous error

For more information about this error, try `rustc --explain E0432`.
Couldn't compile the test.
---- src/cell/lazy.rs - cell::lazy::LazyCell<T,F>::new (line 39) stdout ----
error[E0432]: unresolved import `std::lazy::Lazy`
 --> src/cell/lazy.rs:43:5
6 | use std::lazy::Lazy;
  |     ^^^^^^^^^^^^^^^ no `Lazy` in `lazy`

error: aborting due to previous error
error: aborting due to previous error

For more information about this error, try `rustc --explain E0432`.
Couldn't compile the test.
---- src/cell/lazy.rs - cell::lazy::LazyCell<T,F>::force (line 66) stdout ----
error[E0432]: unresolved import `std::lazy::Lazy`
 --> src/cell/lazy.rs:69:5
6 | use std::lazy::Lazy;
  |     ^^^^^^^^^^^^^^^ no `Lazy` in `lazy`

error: aborting due to previous error
error: aborting due to previous error

For more information about this error, try `rustc --explain E0432`.
Couldn't compile the test.
---- src/cell/once.rs - cell::once::OnceCell (line 12) stdout ----
error[E0432]: unresolved import `std::lazy::OnceCell`
 --> src/cell/once.rs:15:5
6 | use std::lazy::OnceCell;
  |     ^^^^^^^^^^^^^^^^^^^ no `OnceCell` in `lazy`

error: aborting due to previous error
error: aborting due to previous error

For more information about this error, try `rustc --explain E0432`.
Couldn't compile the test.
---- src/cell/once.rs - cell::once::OnceCell<T>::into_inner (line 190) stdout ----
error[E0432]: unresolved import `std::lazy::OnceCell`
 --> src/cell/once.rs:193:5
6 | use std::lazy::OnceCell;
  |     ^^^^^^^^^^^^^^^^^^^ no `OnceCell` in `lazy`

error: aborting due to previous error
error: aborting due to previous error

For more information about this error, try `rustc --explain E0432`.
Couldn't compile the test.
---- src/cell/once.rs - cell::once::OnceCell<T>::set (line 66) stdout ----
error[E0432]: unresolved import `std::lazy::OnceCell`
 --> src/cell/once.rs:69:5
6 | use std::lazy::OnceCell;
  |     ^^^^^^^^^^^^^^^^^^^ no `OnceCell` in `lazy`

error: aborting due to previous error
error: aborting due to previous error

For more information about this error, try `rustc --explain E0432`.
Couldn't compile the test.
---- src/cell/once.rs - cell::once::OnceCell<T>::get_or_try_init (line 144) stdout ----
error[E0432]: unresolved import `std::lazy::OnceCell`
 --> src/cell/once.rs:147:5
6 | use std::lazy::OnceCell;
  |     ^^^^^^^^^^^^^^^^^^^ no `OnceCell` in `lazy`

error: aborting due to previous error
error: aborting due to previous error

For more information about this error, try `rustc --explain E0432`.
Couldn't compile the test.
---- src/cell/once.rs - cell::once::OnceCell<T>::take (line 217) stdout ----
error[E0432]: unresolved import `std::lazy::OnceCell`
 --> src/cell/once.rs:220:5
6 | use std::lazy::OnceCell;
  |     ^^^^^^^^^^^^^^^^^^^ no `OnceCell` in `lazy`

error: aborting due to previous error
error: aborting due to previous error

For more information about this error, try `rustc --explain E0432`.
Couldn't compile the test.
---- src/cell/once.rs - cell::once::OnceCell<T>::get_or_init (line 109) stdout ----
error[E0432]: unresolved import `std::lazy::OnceCell`
 --> src/cell/once.rs:112:5
6 | use std::lazy::OnceCell;
  |     ^^^^^^^^^^^^^^^^^^^ no `OnceCell` in `lazy`

error: aborting due to previous error
error: aborting due to previous error

For more information about this error, try `rustc --explain E0432`.
Couldn't compile the test.

failures:
    src/cell/lazy.rs - cell::lazy::LazyCell (line 9)
    src/cell/lazy.rs - cell::lazy::LazyCell<T,F>::force (line 66)
    src/cell/lazy.rs - cell::lazy::LazyCell<T,F>::new (line 39)
    src/cell/once.rs - cell::once::OnceCell (line 12)
    src/cell/once.rs - cell::once::OnceCell<T>::get_or_init (line 109)
    src/cell/once.rs - cell::once::OnceCell<T>::get_or_try_init (line 144)
    src/cell/once.rs - cell::once::OnceCell<T>::into_inner (line 190)
    src/cell/once.rs - cell::once::OnceCell<T>::set (line 66)
    src/cell/once.rs - cell::once::OnceCell<T>::take (line 217)
test result: FAILED. 3808 passed; 9 failed; 36 ignored; 0 measured; 0 filtered out; finished in 57.84s

error: test failed, to rerun pass '--doc'
Build completed unsuccessfully in 0:22:32
