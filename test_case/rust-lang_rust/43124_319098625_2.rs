rust
        #[inline]
        fn add_one(&self) -> Self {
            self.checked_add(1).expect("Overflow in step!")
        }
