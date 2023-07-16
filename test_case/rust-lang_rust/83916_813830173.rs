plain
    Checking url v2.1.1
    Checking toml v0.5.7
    Checking cargo_metadata v0.12.0
    Checking clippy_lints v0.1.53 (/checkout/src/tools/clippy/clippy_lints)
error[E0026]: variant `rustc_hir::InlineAsmOperand::Const` does not have a field named `expr`
    |
    |
309 |                     | hir::InlineAsmOperand::Const { expr }
    |                                                      ^^^^ variant `rustc_hir::InlineAsmOperand::Const` does not have this field
error[E0027]: pattern does not mention field `anon_const`
   --> src/tools/clippy/clippy_lints/src/utils/inspector.rs:309:23
    |
    |
309 |                     | hir::InlineAsmOperand::Const { expr }
    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing field `anon_const`
help: include the missing field in the pattern
    |
    |
309 |                     | hir::InlineAsmOperand::Const { expr, anon_const }
    |                                                          ^^^^^^^^^^^^^^
help: if you don't care about this missing field, you can explicitly ignore it
    |
309 |                     | hir::InlineAsmOperand::Const { expr, .. }


error[E0026]: variant `rustc_hir::InlineAsmOperand::Const` does not have a field named `expr`
   --> src/tools/clippy/clippy_lints/src/loops/never_loop.rs:145:45
    |
145 |                 | InlineAsmOperand::Const { expr }
    |                                             ^^^^ variant `rustc_hir::InlineAsmOperand::Const` does not have this field
error[E0027]: pattern does not mention field `anon_const`
   --> src/tools/clippy/clippy_lints/src/loops/never_loop.rs:145:19
    |
    |
145 |                 | InlineAsmOperand::Const { expr }
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing field `anon_const`
help: include the missing field in the pattern
    |
    |
145 |                 | InlineAsmOperand::Const { expr, anon_const }
    |                                                 ^^^^^^^^^^^^^^
help: if you don't care about this missing field, you can explicitly ignore it
    |
145 |                 | InlineAsmOperand::Const { expr, .. }

error: aborting due to 4 previous errors

Some errors have detailed explanations: E0026, E0027.
