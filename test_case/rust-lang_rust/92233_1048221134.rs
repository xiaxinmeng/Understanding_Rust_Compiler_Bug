plain
   |
79 |     let param_env = rustc_trait_selection::traits::normalize_param_env_or_error(
   |         ^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_param_env`
   |
   = note: `-D unused-variables` implied by `-D warnings`

error[E0599]: the method `cmp` exists for enum `rustc_middle::mir::ConstraintCategory`, but its trait bounds were not satisfied
     |
     |
2164 |         categorized_path.sort_by(|p0, p1| p0.category.cmp(&p1.category));
     |                                                       ^^^ method cannot be called on `rustc_middle::mir::ConstraintCategory` due to unsatisfied trait bounds
    ::: /checkout/compiler/rustc_middle/src/mir/query.rs:355:1
     |
355  | pub enum ConstraintCategory {
355  | pub enum ConstraintCategory {
     | --------------------------- doesn't satisfy `rustc_middle::mir::ConstraintCategory: Iterator`
     = note: the following trait bounds were not satisfied:
             `rustc_middle::mir::ConstraintCategory: Iterator`
             `rustc_middle::mir::ConstraintCategory: Iterator`
             which is required by `&mut rustc_middle::mir::ConstraintCategory: Iterator`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_borrowck` due to previous error
warning: build failed, waiting for other jobs to finish...
warning: build failed, waiting for other jobs to finish...
error[E0277]: the trait bound `UpvarMigrationInfo: Ord` is not satisfied
     |
1208 |             capture_diagnostic.sort();
1208 |             capture_diagnostic.sort();
     |                                ^^^^ the trait `Ord` is not implemented for `UpvarMigrationInfo`
     |
note: required by a bound in `std::slice::<impl [T]>::sort`
     |
     |
274  |         T: Ord,
     |            ^^^ required by this bound in `std::slice::<impl [T]>::sort`
For more information about this error, try `rustc --explain E0277`.
error: build failed
Build completed unsuccessfully in 0:03:10
