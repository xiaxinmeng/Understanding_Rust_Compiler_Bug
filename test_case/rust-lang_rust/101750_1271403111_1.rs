
│ │ │ ├─106ms DEBUG rustc_borrowck do_mir_borrowck: result = BorrowCheckResult {
│ │ │ │     concrete_opaque_types: VecMap(
│ │ │ │         [
│ │ │ │             (
│ │ │ │                 DefId(0:13 ~ lib[af95]::bar::{opaque#0}),
│ │ │ │                 OpaqueHiddenType {
│ │ │ │                     span: ../playground/tait-bug/src/lib.rs:19:5: 19:34 (#0),
│ │ │ │                     ty: std::iter::Once<(TAIT, Bar)>,
│ │ │ │                 },
│ │ │ │             ),
│ │ │ │             (
│ │ │ │                 DefId(0:11 ~ lib[af95]::TAIT::{opaque#0}),
│ │ │ │                 OpaqueHiddenType {
│ │ │ │                     span: ../playground/tait-bug/src/lib.rs:19:5: 19:34 (#0),
│ │ │ │                     ty: TAIT,
│ │ │ │                 },
│ │ │ │             ),
│ │ │ │             (
│ │ │ │                 DefId(0:14 ~ lib[af95]::bar::{opaque#1}),
│ │ │ │                 OpaqueHiddenType {
│ │ │ │                     span: ../playground/tait-bug/src/lib.rs:19:5: 19:34 (#0),
│ │ │ │                     ty: Bar,
│ │ │ │                 },
│ │ │ │             ),
│ │ │ │         ],
│ │ │ │     ),
│ │ │ │     closure_requirements: None,
│ │ │ │     used_mut_upvars: [],
│ │ │ │     tainted_by_errors: None,
│ │ │ │ }
