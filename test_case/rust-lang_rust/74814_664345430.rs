rust

let not_unwind_safe = std::cell::RefCell::new(42);

let l = std::lazy::SyncLazy::new(|| {
    let mut a = not_unwind_safe.borrow_mut();
    *a = 43;
    
    panic!("lol");
    
    *a
});

std::panic::catch_unwind(|| {
    let a = *l;
});
