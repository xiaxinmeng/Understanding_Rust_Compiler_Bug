rust
#![feature(rustc_attrs, bench_black_box)]

#[rustc_layout_scalar_valid_range_start(1)]
pub struct NonNull {
    pointer: *const (),
}

#[no_mangle]
pub fn use_nonnull(p: &NonNull) {
    std::hint::black_box(p.pointer);
}

fn main() {
    let ptr = 0usize as *const ();
    use_nonnull(unsafe { std::mem::transmute(&ptr) });
}
