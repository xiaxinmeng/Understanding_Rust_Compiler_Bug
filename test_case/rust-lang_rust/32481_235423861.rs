 Rust

mod a {
    pub fn foo() { b::foo() }
    mod b {
        pub fn foo() { c::foo() }
        mod c {
            pub fn foo() { d::foo() }
            mod d {
                pub fn foo() { panic!("Nesting!") }
            }
        }
    }
}
fn main() { a::foo() }
