rust
//revisions:rpass1 rpass2

enum A { 
    B(C),
}

#[cfg(rpass1)]
struct C(Box<A>);

#[cfg(rpass2)]
struct C(A);

fn main() {}
