rust
trait A {
    fn foo();
}

impl<const N: usize> A for Const<N> {
    fn foo() {
        let _: [u8; N + 1];
    }
}

fn test<const N: usize>() where Const<N>: A {
    // ...
} 
