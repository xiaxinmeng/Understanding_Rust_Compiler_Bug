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
Diff in /checkout/compiler/rustc_trait_selection/src/traits/mod.rs at line 56:
 pub use self::object_safety::is_vtable_safe_method;
 pub use self::object_safety::MethodViolationCode;
 pub use self::object_safety::ObjectSafetyViolation;
-pub use self::project::{NormalizeExt, normalize_projection_type};
 pub(crate) use self::project::{normalize, normalize_to};
+pub use self::project::{normalize_projection_type, NormalizeExt};
 pub use self::select::{EvaluationCache, SelectionCache, SelectionContext};
 pub use self::select::{EvaluationResult, IntercrateAmbiguityCause, OverflowError};
 pub use self::specialize::specialization_graph::FutureCompatOverlapError;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_trait_selection/src/traits/coherence.rs" "/checkout/compiler/rustc_trait_selection/src/traits/const_evaluatable.rs" "/checkout/compiler/rustc_codegen_llvm/src/intrinsic.rs" "/checkout/compiler/rustc_trait_selection/src/traits/mod.rs" "/checkout/compiler/rustc_codegen_llvm/src/type_.rs" "/checkout/compiler/rustc_trait_selection/src/traits/wf.rs" "/checkout/compiler/rustc_codegen_llvm/src/common.rs" "/checkout/compiler/rustc_codegen_llvm/src/llvm_util.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
