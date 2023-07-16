plain
    Checking semver v0.11.0
    Checking toml v0.5.7
    Checking clippy_utils v0.1.53 (/checkout/src/tools/clippy/clippy_utils)
    Checking url v2.1.1
error[E0026]: variant `rustc_hir::InlineAsmOperand::Const` does not have a field named `expr`
   --> src/tools/clippy/clippy_utils/src/hir_utils.rs:666:51
    |
666 |                         InlineAsmOperand::Const { expr } | InlineAsmOperand::Sym { expr } => self.hash_expr(expr),
    |                                                   ^^^^ variant `rustc_hir::InlineAsmOperand::Const` does not have this field
error[E0027]: pattern does not mention field `anon_const`
   --> src/tools/clippy/clippy_utils/src/hir_utils.rs:666:25
    |
    |
666 |                         InlineAsmOperand::Const { expr } | InlineAsmOperand::Sym { expr } => self.hash_expr(expr),
    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing field `anon_const`
help: include the missing field in the pattern
    |
    |
666 |                         InlineAsmOperand::Const { expr, anon_const } | InlineAsmOperand::Sym { expr } => self.hash_expr(expr),
    |                                                       ^^^^^^^^^^^^^^
help: if you don't care about this missing field, you can explicitly ignore it
    |
666 |                         InlineAsmOperand::Const { expr, .. } | InlineAsmOperand::Sym { expr } => self.hash_expr(expr),

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0026, E0027.
