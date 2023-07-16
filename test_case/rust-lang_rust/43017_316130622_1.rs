rust
fn main() {
    const P: Path = Path::new("/foo/bar"); // requires #![feature(const_path_new)]
    const U: Unique<i32> = unsafe { Unique::new(0xDEADBEEF) }; // requires #![feature(unique, const_unique_new)]
}
