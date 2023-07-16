rust
struct Foo<const N: usize = 10>;
struct Bar<const N: usize = { usize::MAX - 1 }, const M: usize = N>;

fn foo() -> (Foo, Bar<10>) {
    (Foo::<10>, Bar)
}

struct Baz<const N: usize, const M: usize = { N + 1 }>;
//^ error: generic parameters may not be used in const operations

trait Trait<const N: usize> { const ASSOC: usize; }
pub struct UwU<const N: usize = { <()>::ASSOC }> where (): Trait<N>;
//^ error: no associated item named `ASSOC` found for unit type `()` in the current scope
