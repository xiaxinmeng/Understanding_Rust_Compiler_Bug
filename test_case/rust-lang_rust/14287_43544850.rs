 rust
fn main() {
    let x = &["One", "Two", "Three"];
    std::io::println(unsafe { *x.unsafe_ref(0) });
}
