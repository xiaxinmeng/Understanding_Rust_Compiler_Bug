rust
trait Trait<T> {
    const ASSOC: usize;
}

impl Trait<()> for usize {
    const ASSOC: usize = 2;
}

struct Foo<T, U, const N: usize = { usize::ASSOC }>(T, U)
where
    usize: Trait<T> + Trait<U>;
