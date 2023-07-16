plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
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
Diff in /checkout/compiler/rustc_middle/src/ty/vtable.rs at line 1:
 use std::convert::TryFrom;
 
-use crate::mir::interpret::{alloc_range, AllocId, Allocation, Pointer, Scalar, InterpResult};
+use crate::mir::interpret::{alloc_range, AllocId, Allocation, InterpResult, Pointer, Scalar};
 use crate::ty::fold::TypeFoldable;
 use crate::ty::{self, DefId, SubstsRef, Ty, TyCtxt};
 use rustc_ast::Mutability;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_middle/src/util/common.rs" "/checkout/compiler/rustc_middle/src/util/common/tests.rs" "/checkout/compiler/rustc_middle/src/util/bug.rs" "/checkout/compiler/rustc_middle/src/ty/normalize_erasing_regions.rs" "/checkout/compiler/rustc_middle/src/ty/vtable.rs" "/checkout/compiler/rustc_middle/src/ty/adt.rs" "/checkout/compiler/rustc_middle/src/ty/context.rs" "/checkout/compiler/rustc_middle/src/query/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:13
