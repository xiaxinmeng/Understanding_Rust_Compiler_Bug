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
Diff in /checkout/compiler/rustc_mir/src/interpret/terminator.rs at line 191:
             // Different valid ranges are okay (once we enforce validity,
             // that will take care to make it UB to leave the range, just
             // like for transmute).
-            (abi::Abi::Scalar(caller), abi::Abi::Scalar(callee)) => {
-                caller.value == callee.value
+            (abi::Abi::Scalar(caller), abi::Abi::Scalar(callee)) => caller.value == callee.value,
+            (abi::Abi::ScalarPair(caller1, caller2), abi::Abi::ScalarPair(callee1, callee2)) => {
+                caller1.value == callee1.value && caller2.value == callee2.value
-            (
-                abi::Abi::ScalarPair(caller1, caller2),
-                abi::Abi::ScalarPair(caller1, caller2),
-                abi::Abi::ScalarPair(callee1, callee2),
-            ) => caller1.value == callee1.value && caller2.value == callee2.value,
             _ => false,
         }
         }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir/src/interpret/mod.rs" "/checkout/compiler/rustc_mir/src/interpret/validity.rs" "/checkout/compiler/rustc_mir/src/interpret/cast.rs" "/checkout/compiler/rustc_mir/src/interpret/eval_context.rs" "/checkout/compiler/rustc_mir/src/interpret/intern.rs" "/checkout/compiler/rustc_mir/src/interpret/terminator.rs" "/checkout/compiler/rustc_mir/src/monomorphize/partitioning/mod.rs" "/checkout/compiler/rustc_mir/src/interpret/place.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
