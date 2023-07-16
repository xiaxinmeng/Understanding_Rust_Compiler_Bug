rust
pub struct A;

pub struct B<'a> {
    pub a: &'a A,
}

pub struct C<'a> {
    pub a: &'a A,
}

impl<'a> B<'a> {
    pub fn get_c<'b: 'a>(&self) -> C<'b> { 
        C { a: self.a } 
    }
}
