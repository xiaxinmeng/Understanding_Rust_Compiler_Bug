 rust
struct MyType<T> { v: T }
#[unsafe_destructor]
impl Drop for MyType<u8> {
    fn drop(&mut self) { () }
}

fn main() {
    MyType{v:3u}
}
