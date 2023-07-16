rust
struct S;

impl S {
    fn foo(self) {
        self.foo();
    }
}
