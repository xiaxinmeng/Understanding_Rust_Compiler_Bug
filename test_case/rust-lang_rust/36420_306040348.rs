
error[E0520]: `Container` specializes an item from a parent `impl`, but that item is not marked `default`
  --> test.rs:12:5
   |
7  | / impl<T> Repr for T {
8  | |     type Container = Vec<Self>;
9  | | }
   | |_- parent `impl` is here
...
12 |       type Container = String;
   |       ^^^^^^^^^^^^^^^^^^^^^^^^ cannot specialize default item `Container`
   |
   = note: to specialize, `Container` in the parent `impl` must be marked `default`

error: aborting due to previous error(s)
