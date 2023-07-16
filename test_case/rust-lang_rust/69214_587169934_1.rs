rust
use std::{convert::TryInto, ops::Add};

struct Foo;

impl Add<Foo> for usize {
    type Output = usize;
    
    fn add(self, rhs: Foo) -> Self::Output {
        unimplemented!()
    }
}

impl TryInto<Foo> for usize {
    type Error = ();
    
    fn try_into(self) -> Result<Foo, Self::Error> {
        unimplemented!()
    }
}
