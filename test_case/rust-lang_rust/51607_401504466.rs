 rust
#[global_allocator]
static ALLOC: Alloc = Alloc;

fn main() {
    unsafe `{
        Alloc::init();
        let mut x = Box::new(0);
        ptr::write_volatile(&mut *x, 1);
    }
}
