 rust
struct Container<'a> {
    s: Struct<'a>
}

struct Struct<'a> {
    parent: &'a Struct<'a>
}

impl<'a> Struct<'a> {
    pub fn new() -> Struct<'a> {
        Struct {
            parent: None
        }
    }

    pub fn get(&self, key: &str) -> Container<'a> {
        match self.parent {
            Some(ref p) => p.get(key),
            None => {/* ... */}
        }
    }
}


fn main() {
    //
}
