rust
trait Equal<Other>
where
    Other: ?Sized,
    Other: Equal<Self>,
{
    fn eq(&self, other: &Other) -> bool;
}

default impl <T1, T2> Equal<T1> for T2
where T1: Equal<T2>
{
    fn eq(&self, other: &T1) -> bool {
        other.eq(self)
    }
}

struct A {}
struct B {}

impl Equal<B> for A {
    fn eq(&self, b: &B) -> bool {
        true
    }
}
