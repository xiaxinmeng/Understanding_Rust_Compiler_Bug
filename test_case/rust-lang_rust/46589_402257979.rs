rust


#![feature(nll)]

struct Foo;

impl Foo {
    fn get_self(&mut self) -> Option<&mut Self> {
        Some(self)
    }

    fn new_self(&mut self) -> &mut Self {
        self
    }
    
    fn trigger_bug(&mut self) {
        let other = &mut (&mut *self);
        
        *other = match (*other).get_self() {
            Some(s) => s,
            None => (*other).new_self()
        };
        
        let c = other;
        
    }
}

fn main() {}
