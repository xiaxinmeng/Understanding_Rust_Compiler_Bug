 rust
use std::marker::PhantomData;
use std::sync::Arc;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::mem;

struct ArcCell<T> {
     x: AtomicPtr<T>,
     _marker: PhantomData<Arc<T>>,
}

impl<T> ArcCell<T> {
    fn new(x: Arc<T>) -> ArcCell<T> {
        ArcCell { x: AtomicPtr::new(unsafe {mem::transmute(x)}), _marker: PhantomData }
    }

    fn swap(&self, y: Arc<T>, order: Ordering) -> Arc<T> {
        unsafe {
             mem::transmute(self.x.swap(mem::transmute(y), order))
        }
    }
}
