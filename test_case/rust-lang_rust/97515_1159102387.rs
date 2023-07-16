rust
#![feature(generic_associated_types)]

trait Trait {
    type Gat<'a> where Self: 'a;
    
    fn method<F>(&self, f: F)
    where
        F: for<'a> FnOnce(Self::Gat<'a>);
}

struct Struct;

impl Trait for &Struct {
    type Gat<'a> = &'a Struct;
    
    fn method<F>(&self, f: F)
    where
        F: for<'a> FnOnce(Self::Gat<'a>)
    {
        f(self)
    }
}
