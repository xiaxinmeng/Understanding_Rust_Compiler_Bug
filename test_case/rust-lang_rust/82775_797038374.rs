rust
assert_matches!(rx.next(), Some(Msgs::Specific { a, b: B(b), .. }) if a == b);
