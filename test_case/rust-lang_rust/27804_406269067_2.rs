rust
while let Some(current_one) = hashset_take_arbitrary(&mut to_visit) {
  /* ... let another_one = ... */
  (&mut to_visit).remove(&another_one);
}
