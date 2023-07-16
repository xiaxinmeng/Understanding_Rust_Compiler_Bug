rust
#![feature(decl_macro)]

mod m { pub macro a() {} }

macro a() {} // Outer scope

fn main() {
    use m::*; // Inner scope
    
    a!(); // error[E0659]: `a` is ambiguous (glob import vs any other name from outer scope during import/macro resolution)
}
