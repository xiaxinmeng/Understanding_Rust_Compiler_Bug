rust
#![feature(nll)]

struct Foo;

impl Foo {

    fn get_self(&mut self) -> Option<&mut Self> {
        Some(self)
    }

    fn bad_method(&mut self) {
        let mut var = self;
        
        loop {
            var = match var.get_self() {
                Some(v) => v,
                None => var
            };
        }

    }
    
}

fn main() {}
