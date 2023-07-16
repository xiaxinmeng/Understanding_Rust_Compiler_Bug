rust
#![feature(associated_consts)]

trait MyTrait: Clone {
    const MY_CONST: usize = 4096;
    const MY_OTHER_CONST: i32 = 16;
}

#[derive(Clone)]
struct Dude {
    
}

impl MyTrait for Dude {
    const MY_CONST: u32 = 4096;
    const MY_OTHER_CONST: u32 = 16;
}

fn main() {
    println!("{}", Dude::MY_CONST);
    println!("{}", Dude::MY_OTHER_CONST);
}
