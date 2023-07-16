rust
#![feature(core_intrinsics)] //must be crate-wide ie. #![]
fn print_type_of<T>(_: &T) { //src: https://stackoverflow.com/questions/21747136/how-do-i-print-the-type-of-a-variable-in-rust/29168659#29168659
    println!("{}", unsafe { std::intrinsics::type_name::<T>() });
}

fn main() {
    let a=1.0;// defaulting to f64
    print_type_of(&a);// f64
    println!("{}",a);// 1
    
    let b=-4000000000;// defaulting to i32 despite the 'literal out of range for i32'
    print_type_of(&b);// i32
    println!("{}",b);// 294967296
}
