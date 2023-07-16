rust
struct MyStruct<T>(T);

impl<T> MyStruct<T> {
    #[inline(foo)]
    pub fn f(self) {}
}

fn main() {
    let f = MyStruct(4);
    f.f();  // <-- call this to raise error.
}
