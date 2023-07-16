
#![feature(const_generics)]
#![feature(specialization)]

struct S<const ND:usize>{}

trait Type{
    type tt;
}

default impl<const ND:usize> Type for SSS<ND>{
    type tt=<SSS<{ND-1}> as Type>::tt;
}

impl Type for SSS<0>{
    type tt=i32;
}


fn main() {
    let a=SSS::<3>{};
}
