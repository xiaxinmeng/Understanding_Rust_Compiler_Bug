rust
let x = fun();
assert_matches!(x, Ok(_));
let x = x.unwrap();
assert_eq!(x.name, "Test");
assert_eq!(x.fld, 12);
