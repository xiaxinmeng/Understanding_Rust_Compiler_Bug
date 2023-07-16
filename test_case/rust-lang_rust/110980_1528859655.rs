rust
        // SAFETY: `get_unchecked_mut` is never used to move the `Option` inside `self`.
        // `x` is guaranteed to be pinned because it comes from `self` which is pinned.
        unsafe {
            match Pin::get_unchecked_mut(self) {
               Some(ref mut x) => Some(Pin::new_unchecked(x)),
                None => None,
            }
        }
