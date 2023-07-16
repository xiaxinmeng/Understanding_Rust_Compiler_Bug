rust
mod A {
    use crate::B::*;
    
    fn foo() {
        b();
    }
}

pub mod B {
    // Could suggest use
    fn b() {}
}
