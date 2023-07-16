
error: parameter `T` is never used [--explain E0392]
 --> <anon>:9:10
  |>
9 |> struct B<T: DoesAforB<B<T>>>;
  |>          ^
help: consider removing `T` or using a marker such as `std::marker::PhantomData`

error: aborting due to previous error
