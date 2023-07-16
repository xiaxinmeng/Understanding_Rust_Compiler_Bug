
struct Foo<'a> {
    v: &'a mut (),
}

impl Drop for Foo<'_> {
    fn drop(&mut self) {}
}

fn bar() {
    let mut v = ();
    let mut x = Foo { v: &mut v };
    drop(x);
    x = Foo { v: &mut v };
}
