
#[derive(PartialEq, Eq)]
struct S;

const C: S = S;

impl Drop for S {
    fn drop(&mut self) {
        println!("BOOM");
    }
}

fn main() {
    println!("barrier 1");
    let C = S;
    println!("barrier 2");
    let fresh = S;
    println!("barrier 3");
}

-------------------
barrier 1
BOOM
barrier 2
barrier 3
BOOM
