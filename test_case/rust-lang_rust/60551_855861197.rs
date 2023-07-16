rust
#![feature(const_generics)]
#![feature(const_evaluatable_checked)]

trait WithAConstant {
    const SIZE: usize;
}

struct WithArray<T, U: WithAConstant> where [(); U::SIZE]: {
    data: [T; U::SIZE]
}
