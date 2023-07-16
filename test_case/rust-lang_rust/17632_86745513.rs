 rust
mod tc39 {
    #[derive(Debug, Copy, Eq, PartialEq)]
    pub enum Politics {
        Good,
        Bad,
        Ugly
    }
}

use tc39::Politics::*;

fn main() {
    match Good {
        Good => println!("good!"),
        Bad => println!("bad!"),
        Ugly => println!("ugly!")
    }
}
