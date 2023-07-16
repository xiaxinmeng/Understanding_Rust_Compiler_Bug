rust
use std::cell::Cell;
use std::thread;
use std::panic;

thread_local!(static SILENCE_PANIC: Cell<bool> = Cell::new(false));

#[test]
fn test_panic_hook() {
    let prev = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        if !SILENCE_PANIC.with(|s| s.get()) {
            prev(info);
        }
    }));

    for i in 0..1000 {
        let _ = thread::spawn(move || {
            SILENCE_PANIC.with(|s| s.set(true));
            panic!("Panicked from thread: {}", i);
        }).join();
    }
    println!("All done!");
}
