plain
   Compiling miniz_oxide v0.4.0
   Compiling hashbrown v0.9.0
   Compiling object v0.22.0
   Compiling addr2line v0.14.0
error[E0520]: `lower_bound` specializes an item from a parent `impl`, but that item is not marked `default`
     |
     |
2493 | / impl<T> SizeHint for T {
2494 | |     fn lower_bound(&self) -> usize {
2496 | |     }
2497 | | }
2497 | | }
     | |_- parent `impl` is here
...
2501 | /     fn lower_bound(&self) -> usize {
2502 | |         self.buffer().len()
2503 | |     }
     | |_____^ cannot specialize default item `lower_bound`
     |
     = note: to specialize, `lower_bound` in the parent `impl` must be marked `default`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0520`.
error: could not compile `std`
