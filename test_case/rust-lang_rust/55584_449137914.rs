rust
// The behavior proposed here would change this to "0.12343"
assert_eq!(format!("{:.7?}", 0.1234321111), "0.1234321");
