 rust
fn main() {
    let a = Cons(20, Box::new(Nil));
    let nr:Option<u32> = a.head();
    match nr {
        Some(a) => println!("{}", a),
        None => println!("")
    }
}

use List::*;
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil
}

impl<T> List<T> {
    fn head(self) -> Option<T> {
        match self {
            Cons(nr, _) => Some(nr),
            Nil => None
        }
    }
}
