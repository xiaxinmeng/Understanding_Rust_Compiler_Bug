plain
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
error[E0599]: the method `cmp` exists for enum `rustc_middle::mir::ConstraintCategory`, but its trait bounds were not satisfied
     |
     |
2184 |         categorized_path.sort_by(|p0, p1| p0.category.cmp(&p1.category));
     |                                                       ^^^ method cannot be called on `rustc_middle::mir::ConstraintCategory` due to unsatisfied trait bounds
    ::: /checkout/compiler/rustc_middle/src/mir/query.rs:324:1
     |
324  | pub enum ConstraintCategory {
324  | pub enum ConstraintCategory {
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
1211 |             capture_diagnostic.sort();
1211 |             capture_diagnostic.sort();
     |                                ^^^^ the trait `Ord` is not implemented for `UpvarMigrationInfo`
     |
note: required by a bound in `std::slice::<impl [T]>::sort`
     |
     |
274  |         T: Ord,
     |            ^^^ required by this bound in `std::slice::<impl [T]>::sort`
For more information about this error, try `rustc --explain E0277`.
error: build failed
Build completed unsuccessfully in 0:02:29
