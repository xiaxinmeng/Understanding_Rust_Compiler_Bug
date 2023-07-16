
error: conflicting implementations of trait `Foo` for type `Fizz`: [--explain E0119]
  --> <anon>:19:1
19 |> impl<T> Foo for T
   |> ^
note: conflicting implementation is here:
  --> <anon>:15:1
15 |> impl Foo for Fizz {
   |> ^
