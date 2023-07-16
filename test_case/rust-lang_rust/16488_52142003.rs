 rust
unsafe {
    let mut x = uninitialized();
    some_cxx_function(&mut x);
} // x destructor runs without UB
