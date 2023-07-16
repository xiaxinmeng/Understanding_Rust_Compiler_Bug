
error[E0271]: type mismatch resolving `<std::collections::hash_map::Iter<'_, _, _> as Iterator>::Item == &_`
  --> $DIR/issue-33941.rs:6:36
   |
LL |     for _ in HashMap::new().iter().cloned() {}
   |              ------------   ----   ^^^^^^ expected reference, found tuple
   |              |              |
   |              |              this call is of type `std::collections::hash_map::Iter<'_, _, _>`
   |              this call is of type `&HashMap<_, _>`
