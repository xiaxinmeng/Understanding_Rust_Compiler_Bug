
pub mod a {
    use b::*;
    pub fn foo() {
        bar();
    }
}

pub mod b {
    use a::*;
    pub fn bar() {
        foo();
    }
}

fn main() {
}
