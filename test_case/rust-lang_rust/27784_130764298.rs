 rust
let mut buffer = Vec::with_capacity(alot);
let mut idx = 0;
loop {
     idx += some_char().encode_utf8(&mut buffer[idx..]).unwrap();
}
