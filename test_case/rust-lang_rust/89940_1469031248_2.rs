text
error[E0277]: the trait bound `[Test]: ToOwned` is not satisfied
 --> <source>:5:11
  |
5 |     Owned(<[Test] as ToOwned>::Owned),
  |           ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `ToOwned` is not implemented for `[Test]`
  |
  = help: the following implementations were found:
            <[T] as ToOwned>

error[E0277]: the trait bound `[Test]: ToOwned` is not satisfied in `Test`
   --> <source>:3:10
    |
3   | #[derive(Clone)]
    |          ^^^^^ within `Test`, the trait `ToOwned` is not implemented for `[Test]`
    |
    = help: the following implementations were found:
              <[T] as ToOwned>
    = note: required because it appears within the type `Test`
    = note: this error originates in a derive macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 2 previous errors
