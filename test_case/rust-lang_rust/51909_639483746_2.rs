rust
unsafe {
    [if foo() { ptr::null::<i32>() } else { &42 }, if foo() { &42 } else { ptr::null() }][foo() as usize]
}
