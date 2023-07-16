rust
match opt_value {
    ref mut entry @ None => *entry = Some(x),
    Some(ref entry) => assert_eq!(entry, x),
}