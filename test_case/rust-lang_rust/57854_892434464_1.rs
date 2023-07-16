
error[E0412]: cannot find type `Foo` in this scope
 --> src/lib.rs:1:6
  |
1 | impl Foo {}
  |      ^^^ not found in this scope

error[E0277]: the trait bound `&R: std::io::Read` is not satisfied
   --> src/lib.rs:4:13
    |
4   |     let _ = std::io::Read::read_exact(&mut r, &mut []);
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::io::Read` is not implemented for `&R`
    |
note: required by `read_exact`
help: consider introducing a `where` bound, but there might be an alternative better way to express this requirement
    |
3   | fn bar<R>(r: &R) where &R: std::io::Read {
    |                  ^^^^^^^^^^^^^^^^^^^^^^^

error[E0277]: the trait bound `&R: std::io::Read` is not satisfied
  --> src/lib.rs:5:13
   |
5  |     let _ = std::io::copy(&mut r, unimplemented!());
   |             ^^^^^^^^^^^^^ the trait `std::io::Read` is not implemented for `&R`
   |
help: consider introducing a `where` bound, but there might be an alternative better way to express this requirement
   |
3  | fn bar<R>(r: &R) where &R: std::io::Read {
   |                  ^^^^^^^^^^^^^^^^^^^^^^^

warning: unreachable call
 --> src/lib.rs:5:13
  |
5 |     let _ = std::io::copy(&mut r, unimplemented!());
  |             ^^^^^^^^^^^^^         ---------------- any code following this expression is unreachable
  |             |
  |             unreachable call
  |
  = note: `#[warn(unreachable_code)]` on by default

error[E0277]: the trait bound `&R: std::io::Read` is not satisfied
  --> src/lib.rs:6:37
   |
6  |     let _ = std::io::BufReader::new(r);
   |                                     ^ the trait `std::io::Read` is not implemented for `&R`
   |
note: required by `BufReader::<R>::new`
help: consider introducing a `where` bound, but there might be an alternative better way to express this requirement
   |
3  | fn bar<R>(r: &R) where &R: std::io::Read {
   |                  ^^^^^^^^^^^^^^^^^^^^^^^
