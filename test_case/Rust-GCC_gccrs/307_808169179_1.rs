
error[E0283]: type annotations needed
  --> src/main.rs:10:5
   |
4  |     fn new(x: T) -> Self {
   |     -------------------- required by `Box::<T, A>::new`
...
10 |     Box::new(1);
   |     ^^^^^^^^ cannot infer type for type parameter `A`
   |
   = note: cannot satisfy `_: Default`

error: aborting due to previous error
