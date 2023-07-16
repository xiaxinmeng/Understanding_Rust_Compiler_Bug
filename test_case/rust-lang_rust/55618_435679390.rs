rust
mod T { // T1
    pub macro mac() {}
}
fn f<T>() { // T2
    T::mac!(); // Should be an error because T2 shadows T1, but it's not.
}
