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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_trait_selection/src/traits/const_evaluatable.rs at line 161:
 
     debug!(?concrete, "is_const_evaluatable");
     match concrete {
-        Err(ErrorHandled::TooGeneric) => {
-            Err(match substs.has_infer_types_or_consts() {
-                true => NotConstEvaluatable::MentionsInfer,
-                false => NotConstEvaluatable::MentionsParam,
-        }
-        }
+        Err(ErrorHandled::TooGeneric) => Err(match substs.has_infer_types_or_consts() {
+            true => NotConstEvaluatable::MentionsInfer,
+            false => NotConstEvaluatable::MentionsParam,
+        }),
         Err(ErrorHandled::Linted) => {
             infcx.tcx.sess.delay_span_bug(span, "constant in type had error reported as lint");
             Err(NotConstEvaluatable::Error(ErrorReported))
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_target/src/abi/call/x86.rs" "/checkout/compiler/rustc_trait_selection/src/traits/object_safety.rs" "/checkout/compiler/rustc_target/src/abi/call/msp430.rs" "/checkout/compiler/rustc_trait_selection/src/traits/wf.rs" "/checkout/compiler/rustc_target/src/abi/call/x86_64.rs" "/checkout/compiler/rustc_target/src/abi/call/powerpc.rs" "/checkout/compiler/rustc_target/src/abi/call/s390x.rs" "/checkout/compiler/rustc_trait_selection/src/traits/const_evaluatable.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:16
