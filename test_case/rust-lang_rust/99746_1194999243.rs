plain
help: consider importing one of these items
    |
7   | use crate::traits::FulfillmentContext;
    |
7   | use crate::traits::chalk_fulfill::FulfillmentContext;

error[E0433]: failed to resolve: use of undeclared type `FulfillmentContext`
   --> compiler/rustc_trait_selection/src/traits/specialize/mod.rs:158:13
    |
    |
158 |             FulfillmentContext::new(),
    |             ^^^^^^^^^^^^^^^^^^ not found in this scope
    |
help: consider importing one of these items
    |
13  | use crate::traits::FulfillmentContext;
    |
13  | use crate::traits::chalk_fulfill::FulfillmentContext;

For more information about this error, try `rustc --explain E0433`.
error: could not compile `rustc_trait_selection` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
