 rust
struct Foo {
    n : i32
}

struct Bar<'a> {
    n : i32,
    other : Option<&'a mut Foo>
}

impl<'a> Bar<'a> {
    fn set_foo(&mut self, foo : &'a mut Foo){
        self.other = Some(foo);
    }
}

fn with_bar<F>(func : F) where F: Fn(&mut Bar) {
    let mut bar = Bar { n : 1, other : None };
    func(&mut bar)
}

fn main() {
    let mut foo = Foo { n : 4 };

    with_bar(|bar: &mut Bar| {
        bar.set_foo(&mut foo);
    });
}
