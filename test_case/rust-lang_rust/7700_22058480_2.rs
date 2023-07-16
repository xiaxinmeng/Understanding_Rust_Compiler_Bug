
let mut data = vec::from_elem(1024, 0u8);
...
let mut i = 0;
while i < 512 {
    data[i] = data[512 + i];
    i += 1;
}
