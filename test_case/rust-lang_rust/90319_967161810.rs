
// ... Warnings ...

error[E0614]: type `product::Product` cannot be dereferenced
  --> src/routes/mod.rs:51:22
   |
51 |             .values(&*product_value)
   |                      ^^^^^^^^^^^^^^

error: internal compiler error: compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:856:17: ErrorReporting Overflow should not reach `report_selection_err` call

// ... Please report this ICE ...
