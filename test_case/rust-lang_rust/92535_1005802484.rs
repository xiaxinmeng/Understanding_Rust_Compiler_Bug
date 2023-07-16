rust
#![feature(alloc_error_hook)]

fn main() {
    std::alloc::set_alloc_error_hook(|_| {
        panic!("oh no");
    });
    assert!(std::panic::catch_unwind(|| {
        vec![0u8; 12345678901234567890];
    }).is_err());
    println!("i'm ok :)");
}
