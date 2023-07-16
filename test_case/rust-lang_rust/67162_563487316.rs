rust
assert_eq!(
    format!("{}", Position { longitude: 3., latitude: 7. }),
    "(3, 7)",
);
assert_eq!(
    format!("{:5.2}", Position { longitude: 3., latitude: 7. }),
    "(  3.00,   7.00)",
);
