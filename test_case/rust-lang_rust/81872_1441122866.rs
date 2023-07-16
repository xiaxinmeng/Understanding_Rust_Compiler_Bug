rust
mtx1.with_lock(|guard1| {
    let guard2 = mtx2.lock().unwrap();
    let guard3 = mtx3.lock().unwrap();
}); // lock it all back up
