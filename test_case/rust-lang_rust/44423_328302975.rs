
let s = "this is a string";
let boxed = s.to_owned().into_boxed_str().into_boxed_bytes();
assert_eq!(*boxed, *s.as_bytes());
