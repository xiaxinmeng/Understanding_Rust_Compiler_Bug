
impl<X, Xs, X2> Foo for (Cons<X, Xs>, ())
where
    (Xs, ()): Foo<X=X2>,
    X2: Any,
{
    type X = Tuple;
}
