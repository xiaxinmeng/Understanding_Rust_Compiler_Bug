rust
struct Foo<T>(T);

impl<T> Foo<T> {
    const ASSOC:usize = 4 / std::mem::size_of::<T>();

    fn dummy1(&self) {}

    fn dummy2(&self) {
        if false {
            Self::ASSOC;
        }
    }

    fn dummy3(&self) {
        Self::ASSOC;
    }
}

fn main() {
    // Which of these lines should try to evaluate `Foo::<()>::ASSOC` and cause `const_err`.
    let x: Foo<()> = Foo(()); // does not rn
    x.dummy1(); // does not rn
    x.dummy2(); // does rn (I fairly strongly oppose changing this to a hard error)
    x.dummy3(); // does rn
}
