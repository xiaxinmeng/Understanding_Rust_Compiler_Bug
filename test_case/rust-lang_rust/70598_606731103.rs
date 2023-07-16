rust
fn main() {
    // First create a mutable reference to some memory.
    let ref1: &'static /*0*/ mut u8 = Box::leak(Box::new(42u8));
    // alloc0: Unique(0)
    
    // Then derive a new mutable reference from the previous reference without borrowck knowing about it.
    // alloc0: Unique(0)
    let ref2: &'static /*1*/ mut u8 = unsafe { std::mem::transmute::<&'_ mut u8, &'static mut u8>(&mut *ref1) };
    // alloc0: Unique(0) Unique(1)

    // Next write using one of the references from a new thread.
    std::thread::spawn(move || {
        // alloc0: Unique(0) Unique(1)
        *ref2 = 0;
        // alloc0: Unique(0) Unique(1)
    });

    // Ensure the new thread runs first, so `Unique(1)` doesn't get popped before the new thread runs.
    std::thread::sleep_ms(10);

    // Finally write using the other reference in the current thread.
    // Both writes happened without synchronization, so this is a data race.
    // alloc0: Unique(0) Unique(1)
    *ref1 = 1;
    // alloc0: Unique(0)
    println!("{}", ref1);
}
