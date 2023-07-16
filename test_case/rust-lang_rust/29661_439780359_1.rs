
error[E0277]: the trait bound `<() as Foo>::Inner: std::convert::From<()>` is not satisfied
 --> src/lib.rs:8:6
  |
8 | impl Foo for () {
  |      ^^^ the trait `std::convert::From<()>` is not implemented for `<() as Foo>::Inner`
  |
  = help: the following implementations were found:
            <T as std::convert::From<T>>
  = note: required because of the requirements on the impl of `std::convert::Into<<() as Foo>::Inner>` for `()`
