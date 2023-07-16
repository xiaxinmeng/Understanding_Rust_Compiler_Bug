
error: conflicting implementations of trait `Trait` for type `Staff<_>`: [--explain E0119]
  --> <anon>:20:1
20 |> impl<T: Foo + Baz> Trait for Staff<T> {
   |> ^
note: conflicting implementation is here:
  --> <anon>:16:1
16 |> impl<T: Foo + Bar> Trait for Staff<T> {
   |> ^

error: aborting due to previous error
