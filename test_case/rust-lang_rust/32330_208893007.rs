 Rust
fn conv<T>(t: <T as Iterator>::Item) -> <T as IntoIterator>::Item where T: Iterator {
    t // this function is required due to inference limitations
}
fn foo<T>(t: <T as Iterator>::Item) -> <T as IntoIterator>::Item where T: Iterator + IntoIterator {
    conv::<T>(t)
}
