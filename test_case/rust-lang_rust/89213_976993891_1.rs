rust
struct B {
    x: u32
}

struct A {
    b: B,
    c: u32
}

fn main() {
    let a = A {
        b: B {
            x: 0
        },
        c: 1
    };
    let gen = move || {
        drop(a.b.x);
        yield;
    };
    println!("{}", a.c);
    println!("{}", std::mem::size_of_val(&gen));
}
