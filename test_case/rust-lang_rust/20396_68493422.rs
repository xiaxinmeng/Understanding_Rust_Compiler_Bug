 rust
struct Bar<'a>(FnMut(Option<&()>) + 'a);
fn main() {}                         
