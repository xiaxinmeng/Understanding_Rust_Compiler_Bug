text
Any type parameter or lifetime parameter of an `impl` must meet at least one of
the following criteria:

 - it appears in the _implementing type_ of the impl, e.g. `impl<T> Foo<T>`
 - for a trait impl, it appears in the _implemented trait_, e.g.
   `impl<T> SomeTrait<T> for Foo`
 - it is bound as an associated type, e.g. `impl<T, U> SomeTrait for T
   where T: AnotherTrait<AssocType=U>`
