 rust
#![feature(unboxed_closures, overloaded_calls)]

fn main() {
    let mut x = Some(42u);
    let mut f = |&mut:| { x = None; };
    let p = match x {
        Some(ref p) => p,
        None => fail!(),
    };
    f();
    println!("{}", x);    
    println!("{}", *p);
}
