plain
##[group]Virtual Environment
Environment: ubuntu-18.04
Version: 20210302.0
Included Software: https://github.com/actions/virtual-environments/blob/ubuntu18/20210302.0/images/linux/Ubuntu1804-README.md
Image Release: https://github.com/actions/virtual-environments/releases/tag/ubuntu18%2F
##[group]GITHUB_TOKEN Permissions
Actions: read
Checks: read
Contents: read
---
    Checking semver v0.11.0
    Checking url v2.1.1
    Checking clippy_utils v0.1.52 (/checkout/src/tools/clippy/clippy_utils)
    Checking cargo_metadata v0.12.0
error[E0023]: this pattern has 3 fields, but the corresponding tuple variant has 2 fields
    --> src/tools/clippy/clippy_utils/src/qualify_min_const_fn.rs:175:9
     |
175  |         Rvalue::BinaryOp(_, lhs, rhs) | Rvalue::CheckedBinaryOp(_, lhs, rhs) => {
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 fields, found 3
    ::: /checkout/compiler/rustc_middle/src/mir/mod.rs:2079:5
     |
     |
2079 |     BinaryOp(BinOp, Box<(Operand<'tcx>, Operand<'tcx>)>),
     |     ---------------------------------------------------- tuple variant defined here

error[E0023]: this pattern has 3 fields, but the corresponding tuple variant has 2 fields
    --> src/tools/clippy/clippy_utils/src/qualify_min_const_fn.rs:175:41
     |
175  |         Rvalue::BinaryOp(_, lhs, rhs) | Rvalue::CheckedBinaryOp(_, lhs, rhs) => {
     |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 fields, found 3
    ::: /checkout/compiler/rustc_middle/src/mir/mod.rs:2080:5
     |
     |
2080 |     CheckedBinaryOp(BinOp, Box<(Operand<'tcx>, Operand<'tcx>)>),
     |     ----------------------------------------------------------- tuple variant defined here
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0023`.
error: could not compile `clippy_utils`
