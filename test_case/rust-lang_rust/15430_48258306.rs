
struct Foo {
    a: Bar,
    b: int,
}

struct Bar {
    c: int,
    d: int,
}

impl Deref<Bar> for Foo {
    fn deref<'a>(&'a self) -> &'a Bar {
        &self.a
    }
}

impl Foo {
    fn baz(~self) {
        println!("hello");
    }
}

impl Bar {
    fn baz(&self) {
        println!("goodbye");
    }
}

fn main() {
    let mut f = Foo {
        a: Bar {
            c: 1,
            d: 2,
        },
        b: 3,
    };
    f.baz();
}
