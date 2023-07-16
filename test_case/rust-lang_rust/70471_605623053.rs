rust
#![feature(const_generics)]

struct A<T = u32, const N: usize> {
    arg: T,
}

type C = u64;
const C: usize = 7;

const _: [A<C>; 0] = [];

