text
error[E0275]: overflow evaluating the requirement `Test: Sized`
 --> <source>:5:11
  |
5 |     Owned(<[Test] as ToOwned>::Owned),
  |           ^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: required because of the requirements on the impl of `ToOwned` for `[Test]`

error[E0275]: overflow evaluating the requirement `Test: Sized`
   --> <source>:3:10
    |
3   | #[derive(Clone)]
    |          ^^^^^
    |
    = note: this error originates in a derive macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 2 previous errors
