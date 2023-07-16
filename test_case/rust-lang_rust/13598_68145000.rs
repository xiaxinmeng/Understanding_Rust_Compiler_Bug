 rust
mod a {
    pub mod b {
        pub type C = uint;
    }
}

#[cfg(variant1)]
mod variant1 {
    use a::b;
    use self::b::C as D;

    pub fn main() {
        let d : D = 3;
        println!("d: {}", d);
    }
}

#[cfg(variant2)]
mod variant2 {
    use a::b::C as D;

    pub fn main() {
        let d : D = 3;
        println!("d: {}", d);
    }
}

#[cfg(variant1)]
pub fn main() {
    variant1::main();
}

#[cfg(variant2)]
pub fn main() {
    variant2::main();
}
