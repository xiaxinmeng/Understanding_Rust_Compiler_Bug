rust
#![feature(const_raw_ptr_deref)]

const FOO: &'static VolatileCell<u32> = unsafe { &*(0x4fff780 as *const VolatileCell<u32>) };
struct VolatileCell<T: Copy> {
    marker: std::marker::PhantomData<T>,
}
impl<T: Copy> VolatileCell<T> {
    fn read(&self) -> T {
        unsafe {
            std::ptr::read_volatile(self as *const _ as *const T)
        }
    }
}
