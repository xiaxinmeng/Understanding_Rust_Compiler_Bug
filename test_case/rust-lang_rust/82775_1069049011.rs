rust
let x = assert_matches!(fun(), Ok(x) => x);
assert_eq!(x.name, "Test");
assert_eq!(x.fld, 12);
