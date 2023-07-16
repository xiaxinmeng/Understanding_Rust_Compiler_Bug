rust
trait Foo<const N: usize> {
    const FOO: usize = div2_checked(N);
}

const fn div2_checked(n: usize) -> usize {
    if n % 2 != 0 { panic!(); }
    n/2
}
