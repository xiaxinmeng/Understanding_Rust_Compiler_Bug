rust
fn get_or_init_in_place<'a>(&'a self, f: impl for<'b> FnOnce(&'b mut MaybeUninit<T>)) -> &'a T
