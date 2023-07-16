rust
let mut iter = slice.iter_mut();
while let Some(x) = iter.next() {
    for y in iter.as_slice_mut() {
        // x and y are distinct `&mut T`
    }
}
