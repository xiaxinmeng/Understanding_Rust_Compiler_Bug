rust
trait Hello {
    fn howdy(&self) -> i32;
}

struct Woah { pub field: i32 }

impl Hello for Woah {
    fn howdy(&self) -> i32 { self.field }
}

struct Container {
    pub boxed_trait: Box<Hello>
}

impl Container {
    pub fn new() -> Self {
        Container { boxed_trait: Box::new(Woah) }
    }
}
