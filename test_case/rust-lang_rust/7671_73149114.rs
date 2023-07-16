 rust
struct NoClone;
struct Foo<A, B, C> {
    a: u8,
    b: Option<B>,
    c: C
}

// CURRENT, wrong way to implement clone in deriving
impl<A: Clone, B: Clone, C: Clone> Clone for Foo<A, B, C> {
    fn clone(&self) -> Self {
        Foo {
            a: Clone::clone(&self.a),
            b: Clone::clone(&self.b),
            c: Clone::clone(&self.c),
        }
    }
}

// NEW, Right way to implement clone in deriving
impl<A, B, C> Clone for Foo<A, B, C> where Option<B>: Clone,
                                           C: Clone {
    fn clone(&self) -> Self {
        Foo {
            a: Clone::clone(&self.a),
            b: Clone::clone(&self.b),
            c: Clone::clone(&self.c),
        }
    }
}

fn main() {
   let t: Foo<NoClone, String, &'static str> = Foo { a: 0, b: None, c: "foo" };
   let _ = t.clone();
}
