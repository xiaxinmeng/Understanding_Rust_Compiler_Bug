rust
#![feature(nll)]

struct Foo;

impl Foo {
    fn get_self(&mut self) -> Option<&mut Self> {
        Some(self)
    }
    
    fn match_self(&mut self) -> &mut Self {
        match self.get_self() {
            Some(s) => s,
            None => self.new_self()
        }
    }
    
    fn new_self(&mut self) -> &mut Self {
        self
    }
    
    fn trigger_bug(&mut self) {
        let mut var = self;
        
        // Comment out this loop...
        loop {
            var = var.match_self();
            
            // ...or this statement to make this example compile
            var = match var.get_self() {
                Some(s) => s,
                None => var.new_self()
            }
        }
        
        
    }
}

fn main() {}
