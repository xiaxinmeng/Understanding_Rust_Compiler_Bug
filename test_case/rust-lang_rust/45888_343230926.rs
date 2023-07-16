rust
struct Foo<'a, T: 'a>(&'a mut [T]);

impl<'a, T: PartialOrd + 'a> Foo<'a, T> {
    fn bar(&self) {
        match *self {
            Foo(ref slice) => { *slice < *slice; } //~ERROR cannot borrow data mutably in a `&` reference
        }
    }
}
