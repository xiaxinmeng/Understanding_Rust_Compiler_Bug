rust
struct S { foo: usize }

impl S {
    fn bar(&self) {
        async move {
            self.foo += 1;
        };
    }
}
