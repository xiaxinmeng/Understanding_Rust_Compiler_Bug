rust
mod a {
    pub fn foo() {}
}

mod b {
    pub fn foo() {}
}

mod f {
    pub use crate::a::*;
    pub use crate::b::*;
}

mod g {
    pub use crate::a::*;
    pub use crate::f::*;
}

fn main() {
    g::foo();
}
