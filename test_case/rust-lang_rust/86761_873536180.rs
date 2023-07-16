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
Diff in /checkout/compiler/rustc_mir_build/src/thir/constant.rs at line 60:
     use rustc_apfloat::ieee::{Double, Single};
     let scalar = match fty {
         ty::FloatTy::F32 => {
-            let rust_f = num.parse::<f32>().unwrap_or_else(|e| {
-                panic!("f32 failed to parse `{}`: {:?}", num, e)
-            });
+            let rust_f = num
+                .parse::<f32>()
+                .unwrap_or_else(|e| panic!("f32 failed to parse `{}`: {:?}", num, e));
             let mut f = num.parse::<Single>().unwrap_or_else(|e| {
                 panic!("apfloat::ieee::Single failed to parse `{}`: {:?}", num, e)
             });
Diff in /checkout/compiler/rustc_mir_build/src/thir/constant.rs at line 82:
             Scalar::from_f32(f)
         ty::FloatTy::F64 => {
         ty::FloatTy::F64 => {
-            let rust_f = num.parse::<f64>().unwrap_or_else(|e| {
-                panic!("f64 failed to parse `{}`: {:?}", num, e)
-            });
+            let rust_f = num
+                .parse::<f64>()
+                .unwrap_or_else(|e| panic!("f64 failed to parse `{}`: {:?}", num, e));
             let mut f = num.parse::<Double>().unwrap_or_else(|e| {
                 panic!("apfloat::ieee::Double failed to parse `{}`: {:?}", num, e)
             });
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir_build/src/thir/util.rs" "/checkout/compiler/rustc_traits/src/chalk/lowering.rs" "/checkout/compiler/rustc_mir_build/src/thir/visit.rs" "/checkout/compiler/rustc_traits/src/chalk/db.rs" "/checkout/compiler/rustc_mir_build/src/thir/mod.rs" "/checkout/compiler/rustc_mir_build/src/thir/cx/expr.rs" "/checkout/compiler/rustc_mir_build/src/thir/pattern/const_to_pat.rs" "/checkout/compiler/rustc_mir_build/src/thir/constant.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:11
