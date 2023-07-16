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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_middle/src/ty/mod.rs at line 77:
 };
 pub use self::context::{
     tls, CanonicalUserType, CanonicalUserTypeAnnotation, CanonicalUserTypeAnnotations,
-    CtxtInterners, DelaySpanBugEmitted, FreeRegionInfo, GeneratorDiagnosticData,
+    CtxtInterners, DeducedParamAttrs, DelaySpanBugEmitted, FreeRegionInfo, GeneratorDiagnosticData,
     GeneratorInteriorTypeCause, GlobalCtxt, Lift, OnDiskCache, TyCtxt, TypeckResults, UserType,
-    UserTypeAnnotationIndex, DeducedParamAttrs,
+    UserTypeAnnotationIndex,
 };
 pub use self::instance::{Instance, InstanceDef};
 pub use self::list::List;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_middle/src/ty/error.rs" "/checkout/compiler/rustc_middle/src/ty/mod.rs" "/checkout/compiler/rustc_middle/src/ty/relate.rs" "/checkout/compiler/rustc_middle/src/ty/generics.rs" "/checkout/compiler/rustc_middle/src/ty/cast.rs" "/checkout/compiler/rustc_middle/src/ty/structural_impls.rs" "/checkout/compiler/rustc_middle/src/ty/opaque_types.rs" "/checkout/compiler/rustc_middle/src/ty/parameterized.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
