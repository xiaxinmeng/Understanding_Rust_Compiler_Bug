plain
   Compiling rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
error: expected identifier, found keyword `ref`
    --> compiler/rustc_middle/src/mir/mod.rs:1708:26
     |
1708 |             Coverage(box ref coverage) => write!(fmt, "Coverage::{:?}", coverage.kind),


error: expected one of `)`, `,`, `@`, or `|`, found `coverage`
     |
     |
1708 |             Coverage(box ref coverage) => write!(fmt, "Coverage::{:?}", coverage.kind),
     |                             -^^^^^^^^ expected one of `)`, `,`, `@`, or `|`
     |                             help: missing `,`

   Compiling rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
   Compiling rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
   Compiling rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
   Compiling rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 1 field
     |
     |
1585 |     Coverage(Box<Coverage>),
     |              ------------- tuple variant has 1 field
...
1708 |             Coverage(box ref coverage) => write!(fmt, "Coverage::{:?}", coverage.kind),
     |                      ^^^^^^^ ^^^^^^^^ expected 1 field, found 2
For more information about this error, try `rustc --explain E0023`.
error: could not compile `rustc_middle` due to 3 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
