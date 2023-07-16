rust
let line: Bytes = ...;
match inner.current_state {
  _ if line.starts_with(b"+CUSD: ") => {
    let mut remainder = Cursor::new(&line[9..]);
    let mut ussd_bytes: Vec<u8> = Vec::new();
    remainder.read_until(0x2c, &mut ussd_bytes)?;    //  <-----
    inner.handle_ussd().await?;
  },
  // ...
}
