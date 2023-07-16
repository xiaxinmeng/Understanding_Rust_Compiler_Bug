
error: unconstrained generic constant
  --> library/core/src/iter/adapters/map_windows.rs:16:13
   |
16 |     buffer: Option<[MaybeUninit<I::Item>; 2 * N]>,
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: try adding a `where` bound using this expression: `where [(); 2 * N]:`
