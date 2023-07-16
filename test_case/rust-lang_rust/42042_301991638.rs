rust
struct C([S; 1]);

impl std::ops::Deref for C {
    type Target = [S];

    fn deref<'a>(&'a self) -> &'a [S] { &self.0 }
}

impl std::ops::DerefMut for C {
    fn deref_mut<'a>(&'a mut self) -> &'a mut [S] { &mut self.0 }
}

struct S {}
impl S {
    fn foo(&mut self) {}
}

fn main() {
    let mut c = C([S {}]);
    c[0*0].foo();
}
