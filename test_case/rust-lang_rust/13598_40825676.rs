 rust
mod a {
    pub mod b {
        pub type C = uint;
    }
}

#[cfg(variant1)]
mod variant1 {
    use a::b;
    use D = self::b::C;

    pub fn main() {
        let d : D = 3;
        println!("d: {}", d);
    }
}

#[cfg(variant2)]
mod variant2 {
    use D = a::b::C;

    pub fn main() {
        let d : D = 3;
        println!("d: {}", d);
    }
}

#[cfg(variant3)]
mod variant3 {
    pub use a::b;
    use D = self::b::C;

    pub fn main() {
        let d : D = 3;
        println!("d: {}", d);
    }
}

#[cfg(variant4)]
mod variant4 {
    use D = self::b::C;

    pub fn main() {
        let d : D = 3;
        println!("d: {}", d);
    }

    mod b {
        pub type C = uint;
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

#[cfg(variant3)]
pub fn main() {
    variant3::main();
}

#[cfg(variant4)]
pub fn main() {
    variant4::main();
}
