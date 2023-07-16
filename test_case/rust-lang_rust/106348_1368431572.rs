plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0405]: cannot find trait `Residual` in this scope
   --> library/core/src/iter/traits/double_ended.rs:356:22
    |
356 |         R::Residual: Residual<Option<R::Output>>,
    |
help: consider importing this trait
    |
1   | use crate::ops::Residual;
1   | use crate::ops::Residual;
    |

error[E0412]: cannot find type `ChangeOutputType` in this scope
    |
    |
350 |     ) -> ChangeOutputType<R, Option<R::Output>>
    |
help: consider importing this type alias
    |
    |
1   | use crate::ops::ChangeOutputType;

error[E0433]: failed to resolve: use of undeclared type `FromResidual`
   --> library/core/src/iter/traits/double_ended.rs:364:38
    |
    |
364 |             ControlFlow::Break(r) => FromResidual::from_residual(r),
    |                                      ^^^^^^^^^^^^ use of undeclared type `FromResidual`
help: consider importing this trait
    |
1   | use crate::ops::FromResidual;
    |
