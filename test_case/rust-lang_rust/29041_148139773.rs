 rust
#![feature(const_fn)]

use std::marker::PhantomData;
use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};

pub struct AtomicCell<T> {
    ptr: AtomicPtr<T>,
    phantom: PhantomData<T>,  // std::ptr::Unique is not atomic
}

impl<T> AtomicCell<T> {
    pub const fn new() -> Self {
        AtomicCell {
            ptr: AtomicPtr::new(ptr::null_mut()),
            phantom: PhantomData,
        }
    }

    pub fn set(&self, value: T) {
        self.replace(Some(Box::new(value)));
        // Drop the previous value, if any
    }

    pub fn replace(&self, value: Option<Box<T>>) -> Option<Box<T>> {
        let ptr = match value {
            Some(box_) => Box::into_raw(box_),
            None => ptr::null_mut(),
        };
        opt_box_from_ptr(self.ptr.swap(ptr, Ordering::SeqCst))
    }
}

fn opt_box_from_ptr<T>(ptr: *mut T) -> Option<Box<T>> {
    if ptr.is_null() {
        None
    } else {
        Some(unsafe { Box::from_raw(ptr) })
    }
}

impl<T> AtomicCell<T> {
    pub fn spin_lock<R, F: FnOnce(&mut T) -> R>(&self, f: F) -> R {
        loop {
            let ptr = self.ptr.swap(ptr::null_mut(), Ordering::SeqCst);
            if !ptr.is_null() {
                // Atomic swap gives us exclusive access
                let result = f(unsafe { &mut *ptr });
                // Drop an other value that might have been inserted by another thread.
                drop(opt_box_from_ptr(self.ptr.swap(ptr, Ordering::SeqCst)));
                return result
            }
            // Spin until we swap a non-null
        }
    }
}

unsafe impl<T: Clone> Sync for AtomicCell<T> {}

#[cfg(test)]
mod tests {
    use AtomicCell;
    use std::ops::Add;
    use std::sync::mpsc::{channel, Sender};
    use std::thread;

    static CELL: AtomicCell<Sender<u32>> = AtomicCell::new();

    #[test]
    fn it_works() {
        let (tx, rx) = channel();
        CELL.set(tx);
        for i in 0..10 {
            thread::spawn(move || {
                let tx = CELL.spin_lock(|tx| tx.clone());
                tx.send(i).unwrap();

                // Or even…
                CELL.spin_lock(|tx| tx.send(i).unwrap());
            });
        }
        assert_eq!(rx.iter().take(20).fold(0, Add::add), 45 * 2);
    }
}
