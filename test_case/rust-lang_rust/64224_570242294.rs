rust
#![feature(unwind_attributes)]

extern "C" {
    #[unwind(allow)]
    fn foo();
}

#[cold]
fn drop_box(b: Box<dyn std::any::Any + Send>) {
    drop(b);
}

pub unsafe fn bar() -> i32 {
    std::panic::catch_unwind(|| {
        foo();
        42
    })
    .unwrap_or_else(|e| {
        drop_box(e);
        13
    })
}
