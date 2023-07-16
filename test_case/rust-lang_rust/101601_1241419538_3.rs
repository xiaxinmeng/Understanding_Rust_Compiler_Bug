rust
struct Type<'a, 'b> {
    field: &'b &'a (),
}

impl<'a, 'b> Type<'a, 'b> {
    fn foo() {
        Self::bar()
    }
    
    fn bar() {}
}
