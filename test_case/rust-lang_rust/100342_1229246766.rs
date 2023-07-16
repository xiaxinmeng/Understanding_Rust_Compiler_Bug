rust
unsafe fn init_in_place<T>(f: fn(&mut T)) -> T {
    let mut temp = std::mem::uninitialized();
    f(&mut temp);
    temp
}
