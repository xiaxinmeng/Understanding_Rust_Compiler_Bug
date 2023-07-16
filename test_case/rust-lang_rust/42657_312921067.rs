rust
#![crate_type = "rlib"]

struct Type {
    field1 : u64,
    field2 : u32,
}

impl Type   {
    fn mut_update(&mut self, field2 : u32) -> &mut Self {
        self.field2 = field2;
        self
    }

    fn func_update(self, field2 : u32) -> Self  {
        Type { field2 : field2, .. self }
    }
}
