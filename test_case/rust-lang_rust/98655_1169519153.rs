rust
struct A(u32);

impl PartialEq for A {
    fn eq(&self, other: &A) -> bool {
        self.0 == other.0
    }

    fn ne(&self, _other: &A) -> bool {
        true // Buggy! Doesn't satisfy `!(self == other)`
    }
}

#[derive(PartialEq)]
struct B(A);

fn main() {
    let a1 = A(1);
    let a2 = A(2);
    println!("a1 == a1: {}", a1 == a1);
    println!("a1 != a1: {} (buggy)", a1 != a1);
    println!("a1 == a2: {}", a1 == a2);
    println!("a1 != a2: {}", a1 != a2);
    println!("");

    let b1 = B(A(1));
    let b2 = B(A(2));
    println!("b1 == b1: {}", b1 == b1);
    // - Old behaviour: prints `true`, because the derived `B::ne` calls
    //   `A::ne`, which is buggy.
    // - New behaviour: prints `false`, because the default `B::ne` just
    //   inverts the results of `B::eq`, which is correct.
    println!("b1 != b1: {} (buggy?)", b1 != b1);
    println!("b1 == b2: {}", b1 == b2);
    println!("b1 != b2: {}", b1 != b2);
}
