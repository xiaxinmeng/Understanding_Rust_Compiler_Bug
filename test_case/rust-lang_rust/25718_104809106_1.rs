 rust
let mut start = vec.ptr;
let end = start.offset(vec.len); // does this work if vec.ptr == heap::EMPTY and vec.len == 0?
while start != end {
  let elem = ptr::read(start);
  // ... use elem
  start = start.offset(1);
} 
