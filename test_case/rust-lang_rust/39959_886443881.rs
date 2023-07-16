
impl<X, Xs> Foo for (Cons<X, Xs>, ())
where
    (Xs, ()): Foo,
   <(Xs, ()) as Foo>::X: Any,
{
    type X = Tuple;
}
