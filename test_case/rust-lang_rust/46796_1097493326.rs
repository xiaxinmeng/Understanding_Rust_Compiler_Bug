rust
extern "C" {
    fn platform_panic();
}

pub fn main() {
    std::panic::set_hook(Box::new(|_| unsafe {
        platform_panic();
    }));

    // stuff that might panic
}
