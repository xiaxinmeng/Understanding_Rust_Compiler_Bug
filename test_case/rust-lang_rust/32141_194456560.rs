 Rust
mod Bad {
    pub fn default<T>() -> T where T: Default + ::std::ops::Not<Output=T> {
        !T::default()
    }
}

fn foo<Bad>() -> Bad where Bad: Default + ::std::ops::Not<Output=Bad> {
    Bad::default()
}

fn main() {
    println!("{}", foo::<i32>());
}
