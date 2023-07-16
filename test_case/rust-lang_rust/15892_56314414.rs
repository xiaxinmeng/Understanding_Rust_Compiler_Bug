 rust
#[deriving(Clone)]
enum LinkedList { Cons(Box<uint>), Nil }

impl LinkedList {
    fn tail(self) -> uint {
        match self {
          Nil => 1u,
          Cons(l) => *l.clone()
        }
    }
}

fn main() { }
