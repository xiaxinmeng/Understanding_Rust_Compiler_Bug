
error[E0271]: type mismatch resolving `<std::collections::hash_map::Iter<'_, _, _> as std::iter::Iterator>::Item == &_`
 --> src/main.rs:4:36
  |
4 |     for _ in HashMap::new().iter().cloned() { }
  |                                    ^^^^^^ expected tuple, found reference
  |
  = note: expected type `(&_, &_)`
             found type `&_`

error[E0271]: type mismatch resolving `<std::collections::hash_map::Iter<'_, _, _> as std::iter::Iterator>::Item == &_`
 --> src/main.rs:4:14
  |
4 |     for _ in HashMap::new().iter().cloned() { }
  |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected tuple, found reference
  |
  = note: expected type `(&_, &_)`
             found type `&_`
  = note: required because of the requirements on the impl of `std::iter::Iterator` for `std::iter::Cloned<std::collections::hash_map::Iter<'_, _, _>>`
