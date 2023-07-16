
let mut buf: Vec<u8> = Vec::with_capacity(10000);
buf.set_len(100000); // <-- hard to spot problem is here
file.read_into(buf.as_mut_slice());
return buf;
