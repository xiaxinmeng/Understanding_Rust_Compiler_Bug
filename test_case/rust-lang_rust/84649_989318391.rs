rust
struct VecWrapper<'vec, T> {
   data: &'vec mut [MaybeUninit<T>], // spare capacity of the vec
   len: &'vec mut usize, // points to vec.len field
}

fn push(&mut self, val: T) {
   let (item, rest) = self.capacity.split_first_mut().expect("ran out of capacity");
   item.write(val);
   self.data = rest;
   self.len += 1;
}
