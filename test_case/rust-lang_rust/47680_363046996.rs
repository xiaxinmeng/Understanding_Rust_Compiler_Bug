rust
#![feature(nll)]

#[derive(Debug)]
struct Thing;

impl Thing {
    fn maybe_next(&mut self) -> Option<&mut Self> { None }
}

fn main() {
    let mut thing = Thing;
    let mut temp = &mut thing;
    
    // if let works
    while let Some(next) = temp.maybe_next() {
        temp = next;
    }
    
    println!("{:?}", temp);
}
