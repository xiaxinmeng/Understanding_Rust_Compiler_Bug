
let slice = slice::from_raw_parts_mut(ptr, capacity);
let boxed_slice: Box<[T]> = Box::from_raw(slice);
