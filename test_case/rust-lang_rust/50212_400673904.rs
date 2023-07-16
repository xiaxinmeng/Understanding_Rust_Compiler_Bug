rust
error[E0119]: conflicting implementations of trait `convert::TryFrom<&str>`:
   --> libcore/convert.rs:430:1
    |
421 | impl<T, U> TryFrom<U> for T where T: From<U> {
    | -------------------------------------------- first implementation here
...
430 | impl<'a, T> TryFrom<&'a str> for T where T: ::str::FromStr {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation

error: aborting due to previous error
