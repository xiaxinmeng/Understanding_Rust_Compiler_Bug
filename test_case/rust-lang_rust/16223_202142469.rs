 rust
#[derive(Debug)]
enum E {
    A(String, String),
    B(String, String),
}

impl E {
    fn f(self: Box<Self>) -> Box<Self> {
        match *self { // can't use `match {*self}`, otherwise can't use `self` below.
            E::A(..) => self,  
            E::B(a, b) => Box::new(E::B(b, a)), // <-- E0382 here
        }
    }
}

fn main() {
    println!("{:?}", Box::new(E::A("1".to_owned(), "2".to_owned())).f());
    println!("{:?}", Box::new(E::B("1".to_owned(), "2".to_owned())).f());
}
