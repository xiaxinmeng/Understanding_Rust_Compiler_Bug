rust
#![feature(generic_associated_types)]
#![allow(invalid_type_param_default)]

trait Trait {
    type Assoc<T = u32>;
}

impl Trait for () {
    type Assoc<T> = u32; 
}

impl Trait for u32 {
    type Assoc<T = u32> = T;
}

trait Trait2 {
    type Assoc<T>;
}

impl Trait2 for () {
    type Assoc<T> = u32; 
}

impl Trait2 for u32 {
    type Assoc<T = u32> = T;
}


fn foo<T>() {}
    
fn main() {
    foo::<<() as Trait>::Assoc>(); // ok
    foo::<<u32 as Trait>::Assoc>(); // ok
    
    foo::<<() as Trait2>::Assoc>(); // err
    foo::<<u32 as Trait2>::Assoc>(); // err
}
