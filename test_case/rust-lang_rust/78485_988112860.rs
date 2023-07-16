rust
let mut buf = [MaybeUninit::<u8>::uninit(); 1024];
let mut buf = ReadBuf::uninit(&mut buf);
file.read_buf(&mut buf)?; // a new method which takes `ReadBuf`
let data = buf.filled();
