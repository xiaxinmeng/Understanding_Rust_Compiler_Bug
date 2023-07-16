rust
#![feature(nll)]

#[derive(Default)]
struct X(Vec<u32>);

impl X {
    // error will gone if uncomment
    // fn get_iter(&self) -> Option<std::slice::Iter<'_, u32>> {
    fn get_iter(&self) -> Option<impl Iterator<Item = &u32>> {
        Some(self.0.iter())
    }
    
    fn push(&mut self, x: u32) {
        self.0.push(x);
    }
}

fn main() {
    let mut tmp = X::default();

    match tmp.get_iter() { // `xx` holds the reference to `tmp`
        Some(xx) => {
            for t in xx {
                println!("{:?}", t);
            }
        }
        _yy @ None => {
            tmp.push(1);
        }
        
    };
}
