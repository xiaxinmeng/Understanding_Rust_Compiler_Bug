rust
trait T {}

struct TT<'a> { x: &'a i8 }

impl<'a> T for TT<'a> {}

struct S { data: Vec<Box<T>> }

impl S {
    fn new() -> Self { S{data: Vec::new()} }
    fn push(&mut self, t: Box<T>) { self.data.push(t); }
}

fn boxed_trait_object() {
    let i = 4;
    let t = TT{x: &i};
    let mut s = S::new();
    s.push(Box::new(t)); // Error: `i` does not live long enough
                         // Note: borrowed value must be valid 
                         //   for the static lifetime...
}
