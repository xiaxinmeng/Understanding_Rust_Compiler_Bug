rust
   let mut buf:Vec<u8> = Vec::with_capacity(size);
    buf.resize(size, 0);
   std::io::stdin().read_exact(buf.as_mutable_slice());
