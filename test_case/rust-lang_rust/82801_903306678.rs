Rust
if vec.len() == vec.capacity() {
    unsafe { std::hint::unreachable_unchecked() }
}
