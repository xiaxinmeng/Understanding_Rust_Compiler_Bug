rust
  let elem = 1;
  // assert!(vec![elem, 2] == vec![]);
  let v = vec![elem, 2];
  asset!(v == vec![]);

  // Other potential examples are `env!`, `format!` and `offset_of!`.
