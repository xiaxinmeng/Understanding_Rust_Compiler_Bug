Rust
extern "C" {
    fn call_guest_fn(f: u32) -> u32;
}

#[no_mangle]
fn test_callback() -> u32 {
    42
}

fn main() {
    unsafe { call_guest_fn(test_callback as usize as u32 + 1) };
}
