rust
pub struct ConstNum<const N: usize>;

pub trait Is<const N: usize> {}

impl<const N: usize> Is<N> for ConstNum<N> {}


struct Bar<const N: usize>([u8; N]);

fn foo<const N: usize>(_input: Bar<N>) where ConstNum<N>: Is<2> {
    
}

fn main() {
    foo(Bar([0u8; 2])); //works
    foo(Bar([0u8; 3])); //doesn't
}
