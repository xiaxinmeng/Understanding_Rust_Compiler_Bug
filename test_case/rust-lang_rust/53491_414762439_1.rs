rust
let mut b = MaybeUninit::<bool>::uninitialized();
let bref = b.get_mut();
match bref {
  &b => // insta-UB! We have a bad bool in scope.
}
