rust
struct Foo<const N: usize>;

impl<const N: usize> Foo<N> {
    const FOO: usize = div2_checked(N);
}

const fn div2_checked(n: usize) -> usize {
    if n % 2 != 0 {
        panic!();
    }
    n/2
}

// making this function public will cause a compilation error
fn bar() {
    println!("{:?}", Foo::<1>::FOO);
}
