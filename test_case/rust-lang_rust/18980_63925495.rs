 rust
let mut buf = vec![];

write!(&mut buf, "hello world"); // Calls Vec's impl
write(buf, "hello world"); // Calls &mut [u8]'s impl
