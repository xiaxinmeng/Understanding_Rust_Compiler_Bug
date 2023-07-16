rust
let new_layout = Layout::array::<T>(cap);
finish_grow(new_layout, self.current_memory(), &mut self.alloc);
