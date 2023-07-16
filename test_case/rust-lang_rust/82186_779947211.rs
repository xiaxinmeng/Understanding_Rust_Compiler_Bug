plain
   Compiling cargo_metadata v0.12.0
[RUSTC-TIMING] semver test:false 0.811
   Compiling clippy_lints v0.1.52 (/checkout/src/tools/clippy/clippy_lints)
[RUSTC-TIMING] semver_parser test:false 2.445
error: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
   |
30 | use std::collections::Bound;
   |     ^^^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: `-D deprecated` implied by `-D warnings`

error: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
     |
     |
1431 |                         RangeEnd::Included => Bound::Included(rhs),


error: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
     |
     |
1432 |                         RangeEnd::Excluded => Bound::Excluded(rhs),


error: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
     |
     |
1444 |                         node: (value.clone(), Bound::Included(value)),


error: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
     |
     |
1468 |             (Constant::Int(start), Bound::Included(Constant::Int(end))) => Some(SpannedRange {


error: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
     |
     |
1470 |                 node: (start, Bound::Included(end)),


error: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
     |
     |
1472 |             (Constant::Int(start), Bound::Excluded(Constant::Int(end))) => Some(SpannedRange {


error: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
     |
     |
1474 |                 node: (start, Bound::Excluded(end)),


error: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
     |
     |
1476 |             (Constant::Int(start), Bound::Unbounded) => Some(SpannedRange {


error: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
     |
     |
1478 |                 node: (start, Bound::Unbounded),


error: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
     |
     |
1597 |             (&Kind::End(a, _), &Kind::Start(b, _)) if a != Bound::Included(b) => (),


error: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
     |
     |
1456 |     pub node: (T, Bound<T>),


error: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
     |
     |
1539 |         End(Bound<T>, &'a SpannedRange<T>),


error: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
     |
     |
1549 |         fn value(self) -> Bound<T> {


error: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
     |
     |
1551 |                 Kind::Start(t, _) => Bound::Included(t),


error: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
     |
     |
1566 |                 (Bound::Included(a), Bound::Included(b)) | (Bound::Excluded(a), Bound::Excluded(b)) => a.cmp(&b),


error: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
     |
     |
1566 |                 (Bound::Included(a), Bound::Included(b)) | (Bound::Excluded(a), Bound::Excluded(b)) => a.cmp(&b),


error: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
     |
     |
1566 |                 (Bound::Included(a), Bound::Included(b)) | (Bound::Excluded(a), Bound::Excluded(b)) => a.cmp(&b),


error: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
     |
     |
1566 |                 (Bound::Included(a), Bound::Included(b)) | (Bound::Excluded(a), Bound::Excluded(b)) => a.cmp(&b),


error: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
     |
     |
1568 |                 (Bound::Unbounded, _) | (_, Bound::Unbounded) => unimplemented!(),


error: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
     |
     |
1568 |                 (Bound::Unbounded, _) | (_, Bound::Unbounded) => unimplemented!(),


error: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
     |
     |
1569 |                 (Bound::Included(a), Bound::Excluded(b)) => match a.cmp(&b) {


error: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
     |
     |
1569 |                 (Bound::Included(a), Bound::Excluded(b)) => match a.cmp(&b) {


error: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
     |
     |
1573 |                 (Bound::Excluded(a), Bound::Included(b)) => match a.cmp(&b) {


error: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
     |
     |
1573 |                 (Bound::Excluded(a), Bound::Included(b)) => match a.cmp(&b) {

[RUSTC-TIMING] cargo_metadata test:false 6.155
error: aborting due to 25 previous errors

---
   Compiling toml v0.5.7
[RUSTC-TIMING] serde test:false 5.124
[RUSTC-TIMING] serde test:false 5.243
   Compiling rustc-ap-rustc_span v705.0.0
warning: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
   |
30 | use std::collections::Bound;
   |     ^^^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: `#[warn(deprecated)]` on by default

warning: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
     |
     |
1431 |                         RangeEnd::Included => Bound::Included(rhs),


warning: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
     |
     |
1432 |                         RangeEnd::Excluded => Bound::Excluded(rhs),


warning: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
     |
     |
1444 |                         node: (value.clone(), Bound::Included(value)),


warning: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
     |
     |
1468 |             (Constant::Int(start), Bound::Included(Constant::Int(end))) => Some(SpannedRange {


warning: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
     |
     |
1470 |                 node: (start, Bound::Included(end)),


warning: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
     |
     |
1472 |             (Constant::Int(start), Bound::Excluded(Constant::Int(end))) => Some(SpannedRange {


warning: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
     |
     |
1474 |                 node: (start, Bound::Excluded(end)),


warning: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
     |
     |
1476 |             (Constant::Int(start), Bound::Unbounded) => Some(SpannedRange {


warning: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
     |
     |
1478 |                 node: (start, Bound::Unbounded),


warning: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
     |
     |
1597 |             (&Kind::End(a, _), &Kind::Start(b, _)) if a != Bound::Included(b) => (),


warning: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
     |
     |
1456 |     pub node: (T, Bound<T>),


warning: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
     |
     |
1539 |         End(Bound<T>, &'a SpannedRange<T>),


warning: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
     |
     |
1549 |         fn value(self) -> Bound<T> {


warning: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
     |
     |
1551 |                 Kind::Start(t, _) => Bound::Included(t),


warning: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
     |
     |
1566 |                 (Bound::Included(a), Bound::Included(b)) | (Bound::Excluded(a), Bound::Excluded(b)) => a.cmp(&b),


warning: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
     |
     |
1566 |                 (Bound::Included(a), Bound::Included(b)) | (Bound::Excluded(a), Bound::Excluded(b)) => a.cmp(&b),


warning: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
     |
     |
1566 |                 (Bound::Included(a), Bound::Included(b)) | (Bound::Excluded(a), Bound::Excluded(b)) => a.cmp(&b),


warning: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
     |
     |
1566 |                 (Bound::Included(a), Bound::Included(b)) | (Bound::Excluded(a), Bound::Excluded(b)) => a.cmp(&b),


warning: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
     |
     |
1568 |                 (Bound::Unbounded, _) | (_, Bound::Unbounded) => unimplemented!(),


warning: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
     |
     |
1568 |                 (Bound::Unbounded, _) | (_, Bound::Unbounded) => unimplemented!(),


warning: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
     |
     |
1569 |                 (Bound::Included(a), Bound::Excluded(b)) => match a.cmp(&b) {


warning: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
     |
     |
1569 |                 (Bound::Included(a), Bound::Excluded(b)) => match a.cmp(&b) {


warning: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
     |
     |
1573 |                 (Bound::Excluded(a), Bound::Included(b)) => match a.cmp(&b) {


warning: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
     |
     |
1573 |                 (Bound::Excluded(a), Bound::Included(b)) => match a.cmp(&b) {

   Compiling tokio-util v0.3.1
[RUSTC-TIMING] rustc_ap_rustc_data_structures test:false 2.594
   Compiling proc-macro-crate v0.1.5
---
   Compiling hir_expand v0.0.0 (/checkout/src/tools/rust-analyzer/crates/hir_expand)
[RUSTC-TIMING] chalk_solve test:false 4.240
   Compiling project_model v0.0.0 (/checkout/src/tools/rust-analyzer/crates/project_model)
   Compiling proc_macro_srv v0.0.0 (/checkout/src/tools/rust-analyzer/crates/proc_macro_srv)
warning: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
  --> crates/proc_macro_srv/src/rustc_server.rs:13:24
   |
13 | use std::collections::{Bound, HashMap};
   |
   = note: `#[warn(deprecated)]` on by default


warning: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
   --> crates/proc_macro_srv/src/rustc_server.rs:569:17
    |
569 |         _start: Bound<usize>,


warning: use of deprecated type alias `std::collections::Bound`: moved to `std::ops::Bound`
   --> crates/proc_macro_srv/src/rustc_server.rs:570:15
    |
570 |         _end: Bound<usize>,

   Compiling chalk-recursive v0.56.0
[RUSTC-TIMING] cargo_metadata test:false 7.611
   Compiling hir_def v0.0.0 (/checkout/src/tools/rust-analyzer/crates/hir_def)
---
Dist rust-analyzer-nightly-x86_64-unknown-freebsd
 finished in 10.843 seconds
[TIMING] RustAnalyzer { compiler: Compiler { stage: 1, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } }, target: TargetSelection { triple: "x86_64-unknown-freebsd", file: None } } -- 10.871
Dist llvm-tools-nightly-x86_64-unknown-freebsd
thread 'main' panicked at 'clippy expected to build - essential tool', src/bootstrap/dist.rs:1129:14
 finished in 15.205 seconds
[TIMING] LlvmTools { target: TargetSelection { triple: "x86_64-unknown-freebsd", file: None } } -- 15.356
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-freebsd --target x86_64-unknown-freebsd
Build completed unsuccessfully in 0:26:54
