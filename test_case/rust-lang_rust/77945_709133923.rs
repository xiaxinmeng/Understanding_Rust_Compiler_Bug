rust
unsafe fn write_from_iter<I: Iterator<Item = T>>(&self, mut iter: I, len: usize) -> &mut [T] {
    let mut i = 0;
    let mem = self.ptr.get();
    // Use a manual loop since LLVM manages to optimize it better for
    // slice iterators
    loop {
        let value = iter.next();
        if i >= len || value.is_none() {
            // only advance the pointer when we know for sure how many elements we have
            self.ptr.set(mem.add(len)); // notice `len` here

            // We only return as many items as the iterator gave us, even
            // though it was supposed to give us `len`
           return slice::from_raw_parts_mut(mem, i);
        }
        ptr::write(mem.add(i), value.unwrap());
        i += 1;
    }
}
