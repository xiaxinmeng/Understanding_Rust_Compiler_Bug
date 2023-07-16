rust
trait MyTrait<'a> {
    type Output;
}

fn foo<'a, T>() -> &'a ()
where
    T: MyTrait<'a>, <T as MyTrait<'a>>::Output: 'a,
{
    bar::<T::Output>()
}

fn bar<'a, T>() -> &'a ()
where
    T: 'a,
{
    &()
}

fn main() {}
