rust
use std::{mem, ptr};

use std::future::Future;

async fn ready<T>(t: T) -> T {
    t
}

fn black_box<T>(dummy: T) -> T {
    unsafe {
        let ret = std::ptr::read_volatile(&dummy);
        std::mem::forget(dummy);
        ret
    }
}

#[inline(never)]
fn iter<O, R>(mut routine: R)
where
    R: FnMut() -> O,
{
    loop {
        black_box(routine());
    }
}

pub unsafe fn ready_bench() {
    iter(move || async {
        black_box(ready(42)).await
    });
}
