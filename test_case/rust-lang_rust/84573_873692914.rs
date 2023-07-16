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
Diff in /checkout/compiler/rustc_trait_selection/src/traits/mod.rs at line 15:
 mod on_unimplemented;
 mod project;
 pub mod query;
+pub(crate) mod relationships;
 mod select;
 mod specialize;
 mod structural_match;
Diff in /checkout/compiler/rustc_trait_selection/src/traits/mod.rs at line 21:
 mod util;
 pub mod wf;
-pub(crate) mod relationships;
 
 use crate::infer::outlives::env::OutlivesEnvironment;
 use crate::infer::{InferCtxt, RegionckMode, TyCtxtInferExt};
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_trait_selection/src/traits/util.rs" "/checkout/compiler/rustc_trait_selection/src/traits/mod.rs" "/checkout/compiler/rustc_trait_selection/src/traits/misc.rs" "/checkout/compiler/rustc_trait_selection/src/traits/codegen.rs" "/checkout/compiler/rustc_ast/src/lib.rs" "/checkout/compiler/rustc_trait_selection/src/traits/const_evaluatable.rs" "/checkout/compiler/rustc_ast/src/visit.rs" "/checkout/compiler/rustc_ast/src/token.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:12
