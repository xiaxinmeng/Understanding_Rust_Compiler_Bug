rust
#[derive(Clone)]
struct A;

impl Drop for A {
    fn drop(&mut self) {
        println!("A dropped");
    }
}

#[derive(Clone)]
struct B;

impl Drop for B {
    fn drop(&mut self) {
        println!("B dropped");
    }
}

struct Pair {
    a: A,
    b: B,
}

impl Clone for Pair {
    fn clone(&self) -> Self {
        Pair {
            a: self.a.clone(),
            b: self.b.clone(),
        }
    }
    
    // fn clone_from(&mut self, other: &Self) {
    //     self.b = other.b.clone(); // <-- note: assigned in other order
    //     self.a = other.a.clone();
    // }
}

fn main() {
    let mut first = Pair { a: A, b: B };
    let second = Pair { a: A, b: B };
    first.clone_from(&second);
}
