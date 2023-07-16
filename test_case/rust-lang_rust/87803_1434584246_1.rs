rust 
trait Test {
    fn test<'a: 'a>(arg: &'a str) -> &'a str;
}

trait Lookup {
    type Arg<'a>;
}

impl<T: ?Sized> Lookup for T {
    type Arg<'a> = &'a str;
}

impl Test for () {
    fn test(arg: <Self as Lookup>::Arg<'_>) -> <Self as Lookup>::Arg<'_> {
        arg
    }
}
