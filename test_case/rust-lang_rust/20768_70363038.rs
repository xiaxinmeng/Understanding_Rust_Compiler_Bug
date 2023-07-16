 Rust
#![feature(unboxed_closures)]
fn foo(x: u32) -> u32 { x }

fn main() {
    let x = Fn::<(u32,),u32>::call(&foo, (42,));
    println!("{}", x);
}
