rust
mod m {
    pub trait Trait1 { fn foo() {} }
    pub trait Trait2 { fn bar() {} }
    
    impl Trait1 for u8 {}
    impl Trait2 for u8 {}
}

use m::Trait1 as Trait;

fn main() {
    use m::Trait2 as Trait;
    
    u8::bar(); // OK
    u8::foo(); // OK too, despite Trait2 "shadowing" Trait1
}
