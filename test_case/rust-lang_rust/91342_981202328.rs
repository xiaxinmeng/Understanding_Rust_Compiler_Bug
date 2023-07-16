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
Diff in /checkout/compiler/rustc_middle/src/mir/interpret/error.rs at line 8:
 use rustc_macros::HashStable;
 use rustc_session::CtfeBacktrace;
 use rustc_span::def_id::DefId;
-use rustc_target::abi::{Align, Size, call};
+use rustc_target::abi::{call, Align, Size};
 use std::{any::Any, backtrace::Backtrace, fmt};
 
 #[derive(Debug, Copy, Clone, PartialEq, Eq, HashStable, TyEncodable, TyDecodable)]
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_middle/src/mir/interpret/allocation.rs" "/checkout/compiler/rustc_middle/src/mir/interpret/mod.rs" "/checkout/compiler/rustc_middle/src/mir/interpret/error.rs" "/checkout/compiler/rustc_middle/src/mir/interpret/queries.rs" "/checkout/compiler/rustc_middle/src/mir/interpret/pointer.rs" "/checkout/compiler/rustc_middle/src/mir/interpret/value.rs" "/checkout/compiler/rustc_middle/src/mir/type_foldable.rs" "/checkout/compiler/rustc_middle/src/mir/predecessors.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
