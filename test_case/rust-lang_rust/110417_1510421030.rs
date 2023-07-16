plain
    |                                              ^^^^^^^^^^^^^^^^^^^^ help: a function with a similar name exists: `is_const_evaluable`
    |
   ::: compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:25:1
    |
25  | #[instrument(skip(infcx), level = "debug")]
    | ------------------------------------------- similarly named function `is_const_evaluable` defined here
error[E0425]: cannot find function `is_const_evaluatable` in module `const_evaluatable`
   --> compiler/rustc_trait_selection/src/traits/select/mod.rs:867:46
    |
867 |                     match const_evaluatable::is_const_evaluatable(
867 |                     match const_evaluatable::is_const_evaluatable(
    |                                              ^^^^^^^^^^^^^^^^^^^^ help: a function with a similar name exists: `is_const_evaluable`
    |
   ::: compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:25:1
    |
25  | #[instrument(skip(infcx), level = "debug")]
    | ------------------------------------------- similarly named function `is_const_evaluable` defined here
For more information about this error, try `rustc --explain E0425`.
error: could not compile `rustc_trait_selection` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_trait_selection` due to 2 previous errors
