rust
let tmp = std::mem::replace(self, IoSlice::new(&[]));
let tail = &tmp[cnt..]; // <- lifetime isn't same as `IoSlice<'a>`
*self = IoSlice::new(tail);
