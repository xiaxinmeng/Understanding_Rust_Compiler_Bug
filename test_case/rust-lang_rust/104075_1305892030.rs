rust
struct A(i32);

impl A {
    fn incr_and_ref(&mut self) -> &i32 {
        self.0 += 1;
        &self.0
    }
    
    fn get(&self) -> &i32 {
        &self.0
    }
}

fn main() {
    let a = A(0);
    let a_ref = a.incr_and_ref();
    let a_ref_2 = a.get(); // not allowed, even though in theory we only hold an immutable ref to the inner value of a
    println!("{} {}", a_ref, a_ref_2)
}
