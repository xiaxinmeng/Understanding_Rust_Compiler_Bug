
struct Foo<'a> {
    i: &'a bool,
    j: Option<&'a int>,
}

impl<'a> Foo<'a> {
    fn bar(&mut self, j: &'a int) {
        let child = Foo {
            i: self.i,
            j: Some(j)
        };
    }
}
