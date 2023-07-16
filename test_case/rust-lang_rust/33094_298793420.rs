
error: no method named `foo` found for type `SFoo<()>` in the current scope
  --> <anon>:25:7
   |
25 |     x.foo();
   |       ^^^
   |
   = note: the method `foo` exists but the following trait bounds were not satisfied: `SFoo<()> : Baz`
   = help: items from traits can only be used if the trait is implemented and in scope; the following trait defines an item `foo`, perhaps you need to implement it:
   = help: candidate #1: `HasMethods`

error[E0277]: the trait bound `(): Baz` is not satisfied
  --> <anon>:28:5
   |
28 |     <SFoo<()> as HasMethods>::foo(&x);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Baz` is not implemented for `()`
   |
   = note: required because of the requirements on the impl of `Baz` for `SFoo<()>`
   = note: required by `HasMethods::foo`

