rust
unsafe {
    let mut fast_box: Box<T> = Box::new(std::mem::uninitialized());
    (fast_box.as_mut() as *mut T).write(/* stuff */);
}
