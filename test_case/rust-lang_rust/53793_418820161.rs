compile_fail,E0309
// This won't compile because the applicable impl of
// `SomeTrait` (below) requires that `T: 'a`, but the struct does
// not have a matching where-clause.
struct Foo<'a, T> {
    foo: <T as SomeTrait<'a>>::Out
}

trait SomeTrait<'a> {
    type Output;
}

impl<'a, T> SomeTrait<'a> for T
where
    T: 'a,
{ }
