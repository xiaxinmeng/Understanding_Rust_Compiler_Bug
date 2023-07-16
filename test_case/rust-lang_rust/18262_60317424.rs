 rust
struct Foo<'a> { a: &'a mut Option<Box<uint>> }

trait T {
    fn mutate(&mut self);
}
impl<'a> T for Foo<'a> {
    fn mutate(&mut self) {
        *(self.a) = None;
    }
}

struct TOption<'a> {
    v: Option<Box<T + 'a>>
}

impl<'a> TOption<'a> {
    fn set(&mut self, x: Box<T + 'a>) {
        self.v = Some(x);
    }
    fn mutate(&mut self) {
        self.v.as_mut().map(|t| t.mutate());
    }
}

struct Baz<'a> {
    b: TOption<'a>,
    c: &'a mut Option<Box<uint>>
}

impl<'a> Baz<'a> {
    fn x(&'a mut self) {
        (|| {
            let meh = box Foo { a: self.c };
            self.b.set(meh);
        })();
        let inner = self.c.as_ref().unwrap();
        self.b.mutate();
        println!("{}", inner);
    }
}

fn main() {
    let mut bar = Some(box 1u);
    let mut baz = Baz {
        b: TOption { v: None },
        c: &mut bar
    };
    baz.x();
}
