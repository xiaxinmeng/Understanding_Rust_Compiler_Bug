
#![feature(const_generics)]

fn concat<const N: usize>() -> [i32; N+1] {
    unimplemented!()
}
