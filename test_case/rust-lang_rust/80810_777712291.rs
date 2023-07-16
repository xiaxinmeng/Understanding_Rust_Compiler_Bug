text
Building stage2 tool rustfmt (s390x-unknown-linux-gnu)
   Compiling cc v1.0.60
   Compiling either v1.6.0
error[E0220]: associated type `Item` not found for `L`
   --> /builddir/build/BUILD/rustc-1.50.0-src/vendor/either/src/lib.rs:394:35
    |
394 |         R: IntoIterator<Item = L::Item>,
    |                                   ^^^^ associated type `Item` not found
error[E0220]: associated type `IntoIter` not found for `L`
   --> /builddir/build/BUILD/rustc-1.50.0-src/vendor/either/src/lib.rs:391:41
    |
391 |     pub fn into_iter(self) -> Either<L::IntoIter, R::IntoIter>
    |                                         ^^^^^^^^ associated type `IntoIter` not found
error[E0220]: associated type `IntoIter` not found for `R`
   --> /builddir/build/BUILD/rustc-1.50.0-src/vendor/either/src/lib.rs:391:54
    |
391 |     pub fn into_iter(self) -> Either<L::IntoIter, R::IntoIter>
    |                                                      ^^^^^^^^ associated type `IntoIter` not found
error: aborting due to 3 previous errors
For more information about this error, try `rustc --explain E0220`.
error: could not compile `either`
