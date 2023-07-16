rust
extern crate alloc;

fn main() {
    use alloc::alloc::Layout;

    let layout = Layout::array::<u32>(16).expect("overflow cannot happen");

    let vec = unsafe {
        let alloc = alloc::alloc::alloc(layout).cast::<u32>();
        if alloc.is_null() {
            return;
        }

        alloc.write(1_000_000);

        Vec::from_raw_parts(alloc, 1, 16)
    };

    assert_eq!(vec, &[1_000_000]);
    assert_eq!(vec.capacity(), 16);
}
