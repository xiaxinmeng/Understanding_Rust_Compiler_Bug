rust
// check-pass

struct A;

impl A {
    async fn foo(&self, f: &u32) -> &A { self }
}
