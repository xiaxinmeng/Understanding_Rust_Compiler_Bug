rust
    fn extend_desugared<I: Iterator<Item = T>>(&mut self, iterator: I) {
        // This is the case for a general iterator.
        //
        // This function should be the moral equivalent of:
        //
        //      for item in iterator {
        //          self.push(item);
        //      }

        let (lower, upper) = iterator.size_hint();
        if let Some(upper) = upper {
            self.reserve(upper); // why not ?
        }
        else {
            self.reserve(lower)
        }

        // We use `for_each()` that should allow more efficient code
        iterator.for_each(|element| {
            let len = self.len();
            if len == self.capacity() {
                self.reserve(1);
            }
            unsafe {
                ptr::write(self.get_unchecked_mut(len), element);
                // NB can't overflow since we would have had to alloc the address space
                self.set_len(len + 1);
            }
        })
    }
