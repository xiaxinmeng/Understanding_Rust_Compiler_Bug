 rust
let mut s = io::MemStream::new();
let mut w = io::BufferedWriter::new(s);
try!(w); // call to unwrap()
