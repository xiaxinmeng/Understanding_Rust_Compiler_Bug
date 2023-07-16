plain
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
error: cannot find attribute `label` in this scope
    |
    |
171 |     #[label(borrowck::outlive_constraint_need_borrow_lasts_for)]

error: cannot find attribute `primary_span` in this scope
   --> compiler/rustc_borrowck/src/session_diagnostics.rs:176:11
    |
    |
176 |         #[primary_span]


error[E0433]: failed to resolve: use of undeclared type `RequireBorrowLasts`
    |
    |
261 |                     let sub_label = RequireBorrowLasts::Lasts {
    |                                     ^^^^^^^^^^^^^^^^^^ use of undeclared type `RequireBorrowLasts`
error[E0261]: use of undeclared lifetime name `'a`
   --> compiler/rustc_borrowck/src/session_diagnostics.rs:173:20
    |
    |
170 | pub(crate) enum RequireBorrowLasts {
    |                                   - help: consider introducing lifetime `'a` here: `<'a>`
173 |         category: &'a str,
    |                    ^^ undeclared lifetime

error[E0261]: use of undeclared lifetime name `'a`
error[E0261]: use of undeclared lifetime name `'a`
   --> compiler/rustc_borrowck/src/session_diagnostics.rs:174:23
    |
170 | pub(crate) enum RequireBorrowLasts {
    |                                   - help: consider introducing lifetime `'a` here: `<'a>`
...
174 |         borrow_desc: &'a str,
    |                       ^^ undeclared lifetime
error[E0261]: use of undeclared lifetime name `'a`
   --> compiler/rustc_borrowck/src/session_diagnostics.rs:175:23
    |
    |
170 | pub(crate) enum RequireBorrowLasts {
    |                                   - help: consider introducing lifetime `'a` here: `<'a>`
...
175 |         region_name: &'a RegionName,
    |                       ^^ undeclared lifetime
Some errors have detailed explanations: E0261, E0433.
For more information about an error, try `rustc --explain E0261`.
error: could not compile `rustc_borrowck` due to 6 previous errors
warning: build failed, waiting for other jobs to finish...
