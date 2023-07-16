 rust
#![allow(unused_variables)]

macro_rules! f((v: $x:expr) => ( println!("{:?}", $x) ));

fn foo() {
    macro_rules! g((v: $x:expr) => ( println!("{:?}", $x) ));
    f!(v: 3);
    g!(v: 3);
}

fn bar() {
    let v = 4;
    macro_rules! g((v: $x:expr) => ( println!("{:?} {:?}", $x, v) ));  
    let v = 5;
    f!(v: 3);
    g!(v: 3);
}

fn main() {
    foo();
    bar();
}
