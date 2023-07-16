rust
let old_filled_len = buf.filled_len();
reader.read_buf(buf)?;
let bytes_read = buf.filled_len() - old_filled_len;
