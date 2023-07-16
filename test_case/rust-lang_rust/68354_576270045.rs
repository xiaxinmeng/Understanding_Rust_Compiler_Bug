rust
let mut value = (2, true);
let (mut the_int, _) = &mut value;
the_int = 6;
