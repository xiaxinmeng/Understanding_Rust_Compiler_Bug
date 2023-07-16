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
Diff in /checkout/compiler/rustc_middle/src/ty/sty.rs at line 2396:
             ty::Array(element_ty, _len) => element_ty.is_trivially_pure_clone_copy(),
 
             // A 100-tuple isn't "trivial", so doing this only for reasonable sizes.
-            ty::Tuple(field_tys) => field_tys.len() <= 3
-                && field_tys.iter().all(Self::is_trivially_pure_clone_copy),
+            ty::Tuple(field_tys) => {
+                field_tys.len() <= 3 && field_tys.iter().all(Self::is_trivially_pure_clone_copy)
 
 
             // Sometimes traits aren't implemented for every ABI or arity,
             // because we can't be generic over everything yet.
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_middle/src/ty/generics.rs" "/checkout/compiler/rustc_middle/src/ty/instance.rs" "/checkout/compiler/rustc_middle/src/ty/adt.rs" "/checkout/compiler/rustc_middle/src/ty/binding.rs" "/checkout/compiler/rustc_middle/src/ty/diagnostics.rs" "/checkout/compiler/rustc_middle/src/ty/_match.rs" "/checkout/compiler/rustc_middle/src/ty/print/mod.rs" "/checkout/compiler/rustc_middle/src/ty/sty.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
