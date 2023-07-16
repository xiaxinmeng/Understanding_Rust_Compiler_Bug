 rust
#[repr(C)]
struct X {
    x: u8,
    _alignment: [u64, ..0],
}

fn main() {
    println!("{}", std::mem::pref_align_of::<X>());
    // Prints 8 on x86_64 linux
}
