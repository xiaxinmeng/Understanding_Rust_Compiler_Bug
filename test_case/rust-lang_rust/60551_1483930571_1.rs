rust
#![feature(generic_const_exprs)]

trait WithAConstant {
    const SIZE: usize;
}

struct WithArray<T, U: WithAConstant>
where [String; U::SIZE]: // using any type here has the same result
{
    data: [T; U::SIZE]
}
