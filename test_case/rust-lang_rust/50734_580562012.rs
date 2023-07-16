
error[E0046]: not all trait items implemented, missing: `from_iter`
  --> $DIR/missing-assoc-fn.rs:19:1
   |
LL | impl FromIterator<()> for X {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `from_iter` in implementation
   |
   = help: implement the missing item: `fn from_iter<T>(_: T) -> Self where T: std::iter::IntoIterator, std::iter::IntoIterator::Item = A { unimplemented!() }`
