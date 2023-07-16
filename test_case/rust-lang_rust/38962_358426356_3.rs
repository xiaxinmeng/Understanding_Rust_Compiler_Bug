rust
struct A {}

struct B(A);

impl Borrow<A> for B {
    fn borrow(&self) -> &A {
        &self.0
    }
}

// works despite A not having ToOwned
let d: Cow<A, B>;
