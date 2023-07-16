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
Diff in /checkout/compiler/rustc_codegen_ssa/src/back/link.rs at line 39:
 use std::ffi::OsString;
 use std::fs::{File, OpenOptions};
 use std::io::{BufWriter, Write};
-use std::lazy::{OnceCell, Lazy};
+use std::lazy::{Lazy, OnceCell};
 use std::path::{Path, PathBuf};
 use std::process::{ExitStatus, Output, Stdio};
 use std::{ascii, char, env, fmt, fs, io, mem, str};
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_ssa/src/back/symbol_export.rs" "/checkout/compiler/rustc_codegen_ssa/src/traits/intrinsic.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/write.rs" "/checkout/compiler/rustc_codegen_ssa/src/traits/write.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/link.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/command.rs" "/checkout/compiler/rustc_codegen_ssa/src/traits/coverageinfo.rs" "/checkout/compiler/rustc_codegen_ssa/src/traits/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
