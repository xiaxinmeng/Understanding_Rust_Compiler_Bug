


enum Foo<T> {
    A(T),
    B(Box<Foo<T>>, Box<Foo<T>>),
}

impl<T> Foo<T> {
    fn map<F, T2>(self, mut f: F) -> Foo<T2>
    where
        F: FnMut(T) -> T2,
    {
        match self {
            Foo::A(x) => Foo::A(f(x)),
            Foo::B(x, y) => Foo::B(Box::new(x.map(&mut f)), Box::new(y.map(&mut f))),
        }
    }
}

fn main() {
    Foo::A(0).map(|x| x + 1);
}
