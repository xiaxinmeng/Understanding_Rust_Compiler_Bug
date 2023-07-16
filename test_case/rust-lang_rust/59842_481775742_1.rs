
error[E0623]: lifetime mismatch
  --> src/lib.rs:28:56
   |
26 |     tableau: Tableau<'data_provider, AdaptedMatrixProvider<'original_data, MP>>,
   |              ------------------------------------------------------------------ these two types are declared with different lifetimes...
27 | ) {
28 |     let _: AdaptedMatrixProvider<'original_data, MP> = tableau.provider().clone_with_extra_bound();
   |                                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ...but data from `tableau` flows into `tableau` here

error: aborting due to previous error
