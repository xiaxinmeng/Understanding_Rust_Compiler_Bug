 rust
impl <I: ::std::clone::Clone,
      U: ::std::clone::Clone + IntoIterator,
      F: ::std::clone::Clone>
    ::std::clone::Clone for FlatMap<I, U, F>
    where U::IntoIter: ::std::clone::Clone {
