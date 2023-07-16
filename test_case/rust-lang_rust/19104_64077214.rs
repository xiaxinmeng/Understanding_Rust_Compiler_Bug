 rust
let mut buf: Vec<u8> = vec![];
loop {
    buf.resize(HDR_LEN, 0);
    try!( stream.read_at_least(HDR_LEN, buf[mut]) );
    let msg_len = (... either bit operations on buf or transmute into C struct ...);
    buf.resize(msg_len, 0);
    try!( stream.read_at_least(msg_len, buf[mut]) );
    (... cook the data in buf ...)
}
