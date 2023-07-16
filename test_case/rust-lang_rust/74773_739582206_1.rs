rust
let mut iter = option.splitn(2, '=');
let key = iter.next().unwrap();
let value = iter.next();
