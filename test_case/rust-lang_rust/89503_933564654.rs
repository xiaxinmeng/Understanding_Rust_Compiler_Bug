rust
trait Assoc {
    type B;
}
struct Test<U, T>(U, T);
impl<U, T> Assoc for Test<U, T>
where
    U: Assoc,
    T: Assoc<B = <U as Assoc>::B>,
{
    type B = <U as Assoc>::B;
}
trait TestTrait {}
impl<U, T> TestTrait for Test<U, T>
where
    U: Assoc,
    T: Assoc<B = <Self as Assoc>::B>,
{
}

fn main() {}
