
let mut vec = Vec::with_capacity(1024);
if reader.read_buffer(&mut vec)?.len() != 0 {
    if vec[0] == 0 {
        // ...
    }
}
