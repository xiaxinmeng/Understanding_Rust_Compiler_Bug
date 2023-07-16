rust
// assume `self` is the same as `*const c_char`
let len = sys::strlen(self);
if index.start < len {
    self.offset(index.start)
} else {
    // emulate an out-of-bounds panic
    panic!("index out of bounds: the len is {} but the index is {}", len, index.start");
}
