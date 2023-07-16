rust
struct VirtAddr<T = ()>(*const T);

impl<T> VirtAddr<T> {
    const fn new(ptr: *const T) -> Self {
        Self(ptr)
    }
}

fn main() {
    let a: VirtAddr = VirtAddr::new(std::ptr::null());
    let b: VirtAddr<()> = a;
}
