rust
#![crate_type = "lib"]

pub struct S {
    f: Option<String>
}

impl S {
    pub fn f(&mut self, f: Option<String>) {
        self.f = f;
    }
}
