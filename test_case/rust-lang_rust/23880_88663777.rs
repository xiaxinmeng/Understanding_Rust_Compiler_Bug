
struct Foo<X> {
    x: Box<X>
}

impl<Bar> Foo<Bar> {
    fn foo(&self) {
        type Bar = i32;

        let _: Bar = 42;
    }
}
