
let mut data = vec::from_elem(1024, 0u8);
...
vec::bytes::copy_memory(data, data.const_slice(512, 1024), 512);
