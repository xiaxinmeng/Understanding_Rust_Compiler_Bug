rust
use std::ptr;
use std::time::Instant;

fn main() {
    thread_local!(static PTR: usize = 0);

    const ITERS: u32 = 100_000_000;

    let t0 = Instant::now();
    for _ in 0..ITERS {
        PTR.with(|ptr| unsafe { ptr::write_volatile(ptr as *const _ as *mut _, 0) });
    }
    let dur = t0.elapsed();
    println!("elapsed: {:?}, time/iter: {:?}",
             dur,
             dur.checked_div(ITERS).unwrap());
}
