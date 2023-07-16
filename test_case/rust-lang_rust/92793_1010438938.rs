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
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_builtin_macros/src/deriving/generic/mod.rs at line 185:
 use rustc_ast::{self as ast, BinOpKind, EnumDef, Expr, Generics, PatKind};
 use rustc_ast::{GenericArg, GenericParamKind, VariantData};
 use rustc_attr as attr;
-use rustc_data_structures::map_in_place::MapInPlace;
 use rustc_data_structures::fx::FxHashSet;
+use rustc_data_structures::map_in_place::MapInPlace;
 use rustc_expand::base::{Annotatable, ExtCtxt};
 use rustc_span::symbol::{kw, sym, Ident, Symbol};
 use rustc_span::Span;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_builtin_macros/src/deriving/generic/ty.rs" "/checkout/compiler/rustc_builtin_macros/src/llvm_asm.rs" "/checkout/compiler/rustc_codegen_llvm/src/abi.rs" "/checkout/compiler/rustc_builtin_macros/src/cmdline_attrs.rs" "/checkout/compiler/rustc_codegen_llvm/src/common.rs" "/checkout/compiler/rustc_builtin_macros/src/format_foreign.rs" "/checkout/compiler/rustc_codegen_llvm/src/context.rs" "/checkout/compiler/rustc_builtin_macros/src/deriving/generic/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
