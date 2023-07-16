plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
-    ptr::P,
-    Expr,
-    Path, Stmt,
-};
+use rustc_ast::{ptr::P, Expr, Path, Stmt};
 use rustc_data_structures::fx::FxHashSet;
 use rustc_expand::base::ExtCtxt;
-use rustc_span::{
-    symbol::Ident,
-};
-};
+use rustc_span::{symbol::Ident, Span};
 
 pub(super) struct Context<'cx, 'a> {
     // An optimization.
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_builtin_macros/src/concat_bytes.rs" "/checkout/compiler/rustc_lint/src/levels.rs" "/checkout/compiler/rustc_builtin_macros/src/assert/context.rs" "/checkout/compiler/rustc_builtin_macros/src/deriving/default.rs" "/checkout/compiler/rustc_builtin_macros/src/deriving/clone.rs" "/checkout/compiler/rustc_builtin_macros/src/deriving/debug.rs" "/checkout/compiler/rustc_builtin_macros/src/deriving/bounds.rs" "/checkout/compiler/rustc_builtin_macros/src/format_foreign/printf/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
