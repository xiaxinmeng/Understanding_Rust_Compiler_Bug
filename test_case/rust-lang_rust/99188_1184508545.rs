rust
#![recursion_limit = "100000"]

trait Trait<'a> {
    type Assoc;

    fn method() {}
}

impl<T> Trait<'_> for (T,)
where
    for<'x> T: Trait<'x, Assoc = ()>,
{
    type Assoc = ();
}

impl Trait<'_> for () {
    type Assoc = ();
}

type _4<T> = ((((T,),),),);
type _16<T> = _4<_4<_4<_4<T>>>>;
type _64<T> = _16<_16<_16<_16<T>>>>;
type _128<T> = _64<_64<T>>;

pub fn main() {
    // Something that takes long enough to be measurable.
    type X<T> = _128<T>;

    #[cfg(factor = "1")]
    <X<()> as Trait>::method();
    #[cfg(factor = "2")]
    <X<X<()>> as Trait>::method();
    #[cfg(factor = "3")]
    <X<X<X<()>>> as Trait>::method();
    #[cfg(factor = "4")]
    <X<X<X<X<()>>>> as Trait>::method();
}
