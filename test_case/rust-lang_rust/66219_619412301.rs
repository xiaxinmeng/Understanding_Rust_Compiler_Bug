rust
    unsafe {
        // If the first two elements are out-of-order...
        if len >= 2 && is_less(v.get_unchecked(1), v.get_unchecked(0)) {
            // Read the first element into a stack-allocated variable. If a following comparison
            // operation panics, `hole` will get dropped and automatically write the element back
            // into the slice.
            let mut tmp = mem::ManuallyDrop::new(ptr::read(v.get_unchecked(0)));
            let mut hole = CopyOnDrop { src: &mut *tmp, dest: v.get_unchecked_mut(1) };
            ptr::copy_nonoverlapping(v.get_unchecked(1), v.get_unchecked_mut(0), 1);

            for i in 2..len {
                if !is_less(v.get_unchecked(i), &*tmp) {
                    break;
                }

                // Move `i`-th element one place to the left, thus shifting the hole to the right.
                ptr::copy_nonoverlapping(v.get_unchecked(i), v.get_unchecked_mut(i - 1), 1);
                hole.dest = v.get_unchecked_mut(i);
            }
            // `hole` gets dropped and thus copies `tmp` into the remaining hole in `v`.
        }
    }
