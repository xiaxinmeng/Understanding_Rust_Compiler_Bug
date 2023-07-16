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
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_middle/src/ty/inhabitedness/uninhabited_predicate.rs at line 1:
 use crate::ty::context::TyCtxt;
-use crate::ty::{self,  DefId, DefIdTree, ParamEnv};
+use crate::ty::{self, DefId, DefIdTree, ParamEnv};
 /// Represents whether some type is uninhabited in a given context.
 /// Represents whether some type is uninhabited in a given context.
 /// Examples of uninhabited type are `!`, `enum Void {}`, or a struct containing
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_middle/src/ty/print/mod.rs" "/checkout/compiler/rustc_middle/src/ty/print/pretty.rs" "/checkout/compiler/rustc_middle/src/ty/flags.rs" "/checkout/compiler/rustc_middle/src/lint.rs" "/checkout/compiler/rustc_middle/src/mir/pretty.rs" "/checkout/compiler/rustc_middle/src/ty/inhabitedness/mod.rs" "/checkout/compiler/rustc_middle/src/mir/syntax.rs" "/checkout/compiler/rustc_middle/src/ty/inhabitedness/uninhabited_predicate.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
