rust
trait Combine<T> {
    fn combine(self, other: T);
}

struct S1;
struct S2;

fn foo<C>(x: C)
where
    C: Combine<S1> + Combine<S2>
{
    let s = S1;
    C::combine(x,s1);
}
