rust
#![feature(bool_to_option)]

fn create_value() -> u32 {
    println!("creating");
    16
}

fn main() {
    let cond = false;
    println!("{:?}", cond.then_some(create_value()));
    println!("----");
    println!("{:?}", cond.then(|| create_value())); // or cond.then(create_value)
}
