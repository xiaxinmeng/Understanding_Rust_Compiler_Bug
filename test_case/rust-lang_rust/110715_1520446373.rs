rust
    pub fn new(x: T) -> Self {
        let ptr = Box::into_raw(Box::<T>::new_uninit()).cast::<T>();
        let ret = builtin#shallow_init_box(ptr);
        *ret = ptr;
        ret
    }
