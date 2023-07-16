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
Diff in /checkout/compiler/rustc_abi/src/lib.rs at line 1:
 #![cfg_attr(feature = "nightly", feature(step_trait, rustc_attrs, min_specialization))]
 
-use std::{cmp, fmt};
 #[cfg(feature = "nightly")]
 use std::iter::Step;
 use std::num::{NonZeroUsize, ParseIntError};
Diff in /checkout/compiler/rustc_abi/src/lib.rs at line 7:
 use std::ops::{Add, AddAssign, Mul, RangeInclusive, Sub};
 use std::str::FromStr;
+use std::{cmp, fmt};
 use bitflags::bitflags;
 #[cfg(feature = "nightly")]
Build completed unsuccessfully in 0:00:20
Build completed unsuccessfully in 0:00:20
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_metadata/src/native_libs.rs" "/checkout/compiler/rustc_metadata/src/foreign_modules.rs" "/checkout/compiler/rustc_middle/src/mir/interpret/allocation/provenance_map.rs" "/checkout/compiler/rustc_abi/src/layout.rs" "/checkout/compiler/rustc_abi/src/lib.rs" "/checkout/compiler/rustc_builtin_macros/src/trace_macros.rs" "/checkout/compiler/rustc_fs_util/src/lib.rs" "/checkout/compiler/rustc_metadata/src/lib.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
