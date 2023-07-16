rust
my_slice.chunks(3).map(|s| {
    let mut a = [0; 3];
    a.copy_from_slice(s);
    a
})  // now you have an array_chunks iterator
