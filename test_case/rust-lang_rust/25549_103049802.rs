 rust
fn drop_n_times<T>(val: T, times: u32) {
    struct Holder<T: ?Sized>(T);
    let container = Holder([val]);
    for _ in 0..times {
        &(*(&container as &Holder<[T]>)).0;
    }
}
