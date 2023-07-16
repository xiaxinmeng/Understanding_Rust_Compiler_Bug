rust
#[repr(C)]
struct Struct(f32, f32);

fn huh(vec: Vec<Struct>) -> Vec<f32> {
    let (ptr, len, cap) = vec.into_raw_parts();
    unsafe {
        Vec::from_raw_parts(ptr.cast(), len * 2, cap * 2)
    }
}
