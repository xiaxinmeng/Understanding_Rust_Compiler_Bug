
#[mutable]
struct Nonconst { x: () }
pub fn nonconst() -> Nonconst { Nonconst { x: () } } 

fn x<T: Freeze>(_x: T) { }

fn main() { x((nonconst())) }
