rust
trait Foo {
    type Assoc;
}

trait Bar<'a>: BarStatics<Assoc = <Self as Bar<'a>>::Assoc> {
    type Assoc: 'static;
}

trait BarStatics {
    type Assoc;
}

impl<'a, T> BarStatics for T
where
    T: Bar<'a>,
{
    type Assoc = <Self as BarStatics>::Assoc;
}

struct S<T>(T);

impl<'a, T> Foo for S<T>
where
    T: Bar<'a>,
{
    type Assoc = <T as BarStatics>::Assoc;
}
