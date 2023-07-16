rust
#![feature(maybe_uninit_extra)]

type Block<T> = Box<std::cell::UnsafeCell<[std::mem::MaybeUninit<T>]>>;

pub struct Arena<T> {
    current_block: Block<T>,
    current_block_len: std::cell::Cell<usize>,
    // ...
}

impl<T> Arena<T> {
    pub fn alloc(&self, value: T) -> &mut T {
        let slice = unsafe { &mut *self.current_block.get() };
        let next = self.current_block_len.get();
        if let Some(slot) = slice.get_mut(next) {
            self.current_block_len.set(next + 1);
            slot.write(value)
        } else {
            // Allocate the next block etc.
            unimplemented!()  
        }
    }
}
