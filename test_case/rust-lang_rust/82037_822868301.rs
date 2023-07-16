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
Diff in /checkout/compiler/rustc_session/src/session.rs at line 1:
 use crate::cgu_reuse_tracker::CguReuseTracker;
 use crate::code_stats::CodeStats;
 pub use crate::code_stats::{DataTypeKind, FieldInfo, SizeKind, VariantInfo};
-use crate::config::{
-    self, CrateType, OutputType, PrintRequest, Strip, SwitchWithOptPath,
-};
+use crate::config::{self, CrateType, OutputType, PrintRequest, Strip, SwitchWithOptPath};
 use crate::filesearch;
 use crate::lint::{self, LintId};
 use crate::parse::ParseSess;
Build completed unsuccessfully in 0:00:19
Build completed unsuccessfully in 0:00:19
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_ssa/src/debuginfo/mod.rs" "/checkout/compiler/rustc_session/src/session.rs" "/checkout/compiler/rustc_macros/src/lib.rs" "/checkout/compiler/rustc_macros/src/serialize.rs" "/checkout/compiler/rustc_macros/src/symbols.rs" "/checkout/compiler/rustc_macros/src/session_diagnostic.rs" "/checkout/compiler/rustc_macros/src/type_foldable.rs" "/checkout/compiler/rustc_session/src/search_paths.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
