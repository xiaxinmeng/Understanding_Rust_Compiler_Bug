 rust
mod a {
    #[derive(Debug)]
    pub enum E {
        Variant1,
        Variant2,
    }
}

fn main() {
    let x = a::E::Variant1;
    let y = match x {
        a::E::Variant1 => 1,
        a::E::Variant2 => 2,
    };
    println!("Hello world: {:?}", (x, y));
}

