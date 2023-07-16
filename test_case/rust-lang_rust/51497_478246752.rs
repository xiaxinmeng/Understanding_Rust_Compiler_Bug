rust
mod submodule {
    pub trait MyThing {
        fn foo() {}
    }
    
    pub struct A {}
    
    impl MyThing for A {}
}

use submodule::*;

type MyThing = A;

fn main() {
    MyThing::foo();
}
