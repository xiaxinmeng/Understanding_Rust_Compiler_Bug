Rust
#![allow(unused)]

// Create n.rs using:
// N=1000; for ((i=0; i<N;i++)); do echo "impl Foo { fn foo_$i() {} }"; done > n.rs
// (differing N allowed)

// Include n.rs 10 times
mod n_0 { struct Foo {} include!("n.rs"); }
mod n_1 { struct Foo {} include!("n.rs"); }
mod n_3 { struct Foo {} include!("n.rs"); }
mod n_4 { struct Foo {} include!("n.rs"); }
mod n_5 { struct Foo {} include!("n.rs"); }
mod n_6 { struct Foo {} include!("n.rs"); }
mod n_7 { struct Foo {} include!("n.rs"); }
mod n_8 { struct Foo {} include!("n.rs"); }
mod n_9 { struct Foo {} include!("n.rs"); }

fn main() {}
