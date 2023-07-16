rust
assert_eq!("\u{2764}".parse::<char>().unwrap(), '\u{2764}');
assert_eq!("\x40".parse::<char>().unwrap(), '\x40');
