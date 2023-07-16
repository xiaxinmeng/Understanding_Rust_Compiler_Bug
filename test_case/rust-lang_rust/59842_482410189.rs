
$ rustc file.rs -Zborrowck=mir
error: lifetime may not live long enough
  --> file.rs:28:12
   |
25 | fn create_and_solve_subproblems<'data_provider, 'original_data, MP>(
   |                                 --------------  -------------- lifetime `'original_data` defined here
   |                                 |
   |                                 lifetime `'data_provider` defined here
...
28 |     let _: AdaptedMatrixProvider<'original_data, MP> = tableau.provider().clone_with_extra_bound();
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'data_provider` must outlive `'original_data`

error[E0597]: `tableau` does not live long enough
  --> file.rs:28:56
   |
25 | fn create_and_solve_subproblems<'data_provider, 'original_data, MP>(
   |                                                 -------------- lifetime `'original_data` defined here
...
28 |     let _: AdaptedMatrixProvider<'original_data, MP> = tableau.provider().clone_with_extra_bound();
   |            -----------------------------------------   ^^^^^^^ borrowed value does not live long enough
   |            |
   |            type annotation requires that `tableau` is borrowed for `'original_data`
29 | }
   | - `tableau` dropped here while still borrowed

error: aborting due to 2 previous errors
