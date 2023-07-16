
error[E0034]: multiple applicable items in scope
 --> src/main.rs:2:29
  |
2 |     println!("{:?}", (0..0).is_empty());
  |                             ^^^^^^^^ multiple `is_empty` found
  |
  = note: candidate #1 is defined in an impl for the type `std::ops::Range<_>`
note: candidate #2 is defined in the trait `std::iter::ExactSizeIterator`
  = help: to disambiguate the method call, write `std::iter::ExactSizeIterator::is_empty(::std::ops::Range{start: 0, end: 0,})` instead
