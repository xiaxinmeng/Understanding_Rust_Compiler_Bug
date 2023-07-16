Rust
#[derive(Debug)]
struct A<'a> {
    name: &'a str
}

#[derive(Debug)]
struct B<'a, 'b> {
    a_ref1: &'a mut A<'a>,
    a_ref2: &'b mut A<'b>
}

fn main() {
    let mut a1 = A {
        name: "Denis"
    };
    let mut a2 = A {
        name: "Denis"
    };
    let b = B {
        a_ref1: &mut a1,
        a_ref2: &mut a2
    };
    println!("B: {:?}", b);
}
