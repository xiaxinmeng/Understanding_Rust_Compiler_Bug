
#![feature(fnbox)]

use std::boxed::FnBox;

fn main() { 
    let mut x = 3;
    let mut f: Option<Box<FnBox() -> ()>> = None;
    let g = || {
        println!("x {}", x);
        f.unwrap()();
    };
    x = 4; // illegal, x is borrowed by g
    f = Some(Box::new(|| { println!("inside f")})); // legal!!!!!!!!!!!!!!!!!
    g();
}
