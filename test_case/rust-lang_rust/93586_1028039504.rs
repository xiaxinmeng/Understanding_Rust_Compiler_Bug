rust
fn main() {
    let path = std::path::Path::new("C:");
    let mut iter = path.components();
    iter.next();
    iter.as_path(); // panics at 'range start index 2 out of range for slice of length 0'
}
