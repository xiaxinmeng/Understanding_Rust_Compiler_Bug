rust
#![feature(nll)]

#[derive(Default)]
struct X(Vec<u32>);

struct DropIter<'a, X: 'a>(std::slice::Iter<'a, X>);

// (just delegates to underlying iterator)
impl<'a, X> Iterator for DropIter<'a, X> {
    type Item = &'a X;
    fn next(&mut self) -> Option<&'a X> { self.0.next() }
}

impl<'a, X> Drop for DropIter<'a, X> {
    fn drop(&mut self) { println!("dropped here"); }
}

impl X {
    // error will gone if uncomment
    // fn get_iter(&self) -> Option<std::slice::Iter<'_, u32>> { Some(self.0.iter()) }
    fn get_iter(&self) -> Option<DropIter<'_, u32>> { Some(DropIter(self.0.iter())) }

    fn push(&mut self, x: u32) {
        self.0.push(x);
    }
}

fn main() {
    let mut tmp = X::default();
    tmp.push(1);

    println!("before if let");
    match tmp.get_iter() { // `xx` holds the reference to `tmp`
        Some(xx) => {
            println!("before then for loop");
            for t in xx {
                println!("{:?}", t);
            }

            // but here `xx` is dropped
            println!("end of then")
        }
        xx @ None => {
            drop(xx); // this tells borrow-checker to release the borrow of tmp
            tmp.push(1);
            println!("end of else")
        }
    }
    println!("after if let");
}
