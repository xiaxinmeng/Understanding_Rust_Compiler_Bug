plain
    Checking clippy_lints v0.1.66 (/checkout/src/tools/clippy/clippy_lints)
error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
    --> src/tools/clippy/clippy_lints/src/suspicious_operation_groupings.rs:585:17
     |
585  |         | (Loop(_, _), Loop(_, _))
     |
    ::: /checkout/compiler/rustc_ast/src/ast.rs:1364:10
     |
     |
1364 |     Loop(P<Block>, Option<Label>, Span),
     |
help: use `_` to explicitly ignore each field
     |
     |
585  |         | (Loop(_, _, _), Loop(_, _))
help: use `..` to ignore all fields
     |
     |
585  |         | (Loop(..), Loop(_, _))

error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
    --> src/tools/clippy/clippy_lints/src/suspicious_operation_groupings.rs:585:29
     |
     |
585  |         | (Loop(_, _), Loop(_, _))
     |
    ::: /checkout/compiler/rustc_ast/src/ast.rs:1364:10
     |
     |
1364 |     Loop(P<Block>, Option<Label>, Span),
     |
help: use `_` to explicitly ignore each field
     |
     |
585  |         | (Loop(_, _), Loop(_, _, _))
help: use `..` to ignore all fields
     |
     |
585  |         | (Loop(_, _), Loop(..))

For more information about this error, try `rustc --explain E0023`.
error: could not compile `clippy_lints` due to 2 previous errors
Build completed unsuccessfully in 0:04:54
