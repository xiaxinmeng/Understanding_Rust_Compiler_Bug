 rust
struct S<T>(T);

impl <T: Clone> Clone for S<T> {
    fn clone(&self) -> S<T> {
        match *self {
            S(ref x) => S(Clone::clone(&*x)),
        }
    }
}

impl <T: Copy> Copy for S<T> { }

fn main() { }
