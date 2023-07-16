rust
pub struct MyAtomicU8(UnsafeCell<u8>);

unsafe impl Sync for MyAtomicU8 {}

impl MyAtomicU8 {
    pub fn store(&self, value: u8, ordering: Ordering) {
        fence(ordering);
        unsafe { self.0.get().write(value) }
    }

    pub fn load(&self, ordering: Ordering) -> u8 {
        let value = unsafe { self.0.get().read() };
        fence(ordering);
        value
    }
}
