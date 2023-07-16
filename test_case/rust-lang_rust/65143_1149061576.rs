rust
if my_expensive_thing.is_borrowed() {
  return Err(MyError::AttemptToCloneExpensiveThing);
} else {
  do_something_heavy(my_expensive_thing.to_mut());
}
