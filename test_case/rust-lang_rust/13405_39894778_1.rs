
struct Foo<'a> {
    i: &'a bool,
    j: Option<&'a int>,
}

impl<'a> Foo<'a> {
    fn bar<'b>(&'b mut self, j: &'b int) {
        let child = Foo {
            i: self.i,
            j: Some(j)
        };
    }
}
