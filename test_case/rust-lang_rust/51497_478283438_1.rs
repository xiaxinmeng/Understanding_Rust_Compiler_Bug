rust
mod m {
    pub trait Trait1 { fn foo() {} }
    
    impl Trait1 for u8 {}
}
mod n {
    pub trait Trait2 { fn foo() {} }
    
    impl Trait2 for u8 {}
}


use m::Trait1;

fn main() {
    use n::*;
    
    u8::foo(); // Now: ERROR multiple applicable items in scope
               // Proposed: OK, Trait1::foo is selected despite Trait2 being
               //           "closer in scopes" and "shadowing" Trait1.
}
