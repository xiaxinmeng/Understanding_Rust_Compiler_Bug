 rust
struct TestStruct {
    member: Box<Foo>
}

// Compiles
impl PartialEq for TestStruct {
    fn eq(&self, rhs: &Self) -> bool {
        &self.member == &rhs.member
    }
}

//  ERROR: cannot move out of borrowed content
impl PartialEq for TestStruct {
    fn eq(&self, rhs: &Self) -> bool {
        self.member == rhs.member
    }
}
