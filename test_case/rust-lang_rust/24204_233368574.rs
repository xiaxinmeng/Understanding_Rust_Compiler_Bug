
error: type mismatch resolving `<<T as Trait>::A as MultiDispatch<i32>>::O == T`:
 expected associated type,
    found type parameter [--explain E0271]
  --> <anon>:14:1
   |>
14 |> fn test<T: Trait<B=i32>>(b: i32) -> T where T::A : MultiDispatch<i32> { T::new(b) }
   |> ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: required by `Trait`
