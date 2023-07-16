plain
    Checking askama_shared v0.12.0
   Compiling askama_derive v0.11.0
    Checking askama v0.11.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0531]: cannot find tuple struct or tuple variant `ReEmpty` in module `ty`
   --> src/librustdoc/clean/mod.rs:238:19
    |
238 |             | ty::ReEmpty(_)
    |                   ^^^^^^^ not found in `ty`

error[E0599]: no method named `is_empty` found for reference `&rustc_middle::ty::Region<'tcx>` in the current scope
   --> src/librustdoc/clean/mod.rs:332:14
    |
332 |         if a.is_empty() && b.is_empty() {
    |              ^^^^^^^^ method not found in `&rustc_middle::ty::Region<'tcx>`

error[E0599]: no method named `is_empty` found for reference `&rustc_middle::ty::Region<'tcx>` in the current scope
   --> src/librustdoc/clean/mod.rs:332:30
    |
332 |         if a.is_empty() && b.is_empty() {
    |                              ^^^^^^^^ method not found in `&rustc_middle::ty::Region<'tcx>`

error[E0599]: no method named `is_empty` found for reference `&rustc_middle::ty::Region<'tcx>` in the current scope
   --> src/librustdoc/clean/mod.rs:349:15
    |
349 |         if lt.is_empty() {
    |               ^^^^^^^^ method not found in `&rustc_middle::ty::Region<'tcx>`
Some errors have detailed explanations: E0531, E0599.
For more information about an error, try `rustc --explain E0531`.
error: could not compile `rustdoc` due to 4 previous errors
Build completed unsuccessfully in 0:03:53
