 rust
mod a {
    pub fn foo() {
        println!("{}", line!())
    }
    pub mod b {
        pub fn foo() {
            println!("{}", line!())
        }
    }
}
mod test1 {
    pub use super::a::foo;
}
mod test2 {
    pub use super::a::b::foo;
}
fn main() {
    test1::foo();
    test2::foo()
}
