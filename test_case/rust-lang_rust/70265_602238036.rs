rust
struct S<'a> { x: &'a str }

impl S<'static> { fn f(self) {} }

impl<'a> S<'a> {
    fn f2() -> S<'static> {
        S::<'static>{x: ""} // Lifetime annotation in function body
    }
  
    fn g2() {
        S::f(S::<'static>{x: ""})  // Lifetime annotation in function body
    }
}

fn main() {}
