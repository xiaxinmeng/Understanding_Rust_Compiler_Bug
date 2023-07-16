 rust
let mut it = x.offsets('"');
x.slice(it.next().unwrap(), it.next().unwrap() + 1)
