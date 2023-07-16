rust
let iter = BitmapIter::new(slice, 0, slice.len() * 8).map(|x| x as usize);
