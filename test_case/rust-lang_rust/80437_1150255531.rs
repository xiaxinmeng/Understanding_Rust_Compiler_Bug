rust
let c = Box::new(5);

assert_eq!(Box::into_inner(c), 5);
