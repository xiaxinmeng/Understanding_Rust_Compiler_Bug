rust
#![feature(type_name_of_val)]

fn main() {
    let x: u32 = 3;
    let y: &dyn Send = &x;
    println!("{}", std::any::type_name_of_val(y)); // prints something like 'dyn Send'
}
