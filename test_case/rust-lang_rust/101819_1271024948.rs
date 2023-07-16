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
Diff in /checkout/compiler/rustc_target/src/abi/mod.rs at line 1195:
     ) -> Option<Self> {
         let niche_data = |offset, scalar| {
             let Scalar::Initialized {value, valid_range } = scalar else { return None };
-            Some(NicheData {
-                value,
-                valid_range,
-            })
-            })
+            Some(NicheData { offset, value, valid_range })
 
 
         let niche = Niche {
Diff in /checkout/compiler/rustc_target/src/abi/mod.rs at line 1206:
             data: niche_data(niche_offset, niche_scalar)?,
-            flag: Some(
-                niche_data(flag_offset, flag_scalar)?,
-            )
+            flag: Some(niche_data(flag_offset, flag_scalar)?),
 
 
         if niche.available(cx) > 0 || niche.available_with_new_flag(cx) > 0 {
Diff in /checkout/compiler/rustc_target/src/abi/mod.rs at line 1237:
                 let niche_size = self.data.value.size(cx);
                 assert!(niche_size.bits() <= 128);
                 let max = size.unsigned_int_max();
-                if max < u128::MAX {
-                    max + 1
-                    max
-                }
-                }
+                if max < u128::MAX { max + 1 } else { max }
                 0
             }
Diff in /checkout/compiler/rustc_target/src/abi/mod.rs at line 1248:
         })
         })
     }
 
-    pub fn reserve_with_new_flag<C: HasDataLayout>(&self, cx: &C, count: u128) -> Option<(u128, Scalar, Scalar)> {
+    pub fn reserve_with_new_flag<C: HasDataLayout>(
+        &self,
+        cx: &C,
+        count: u128,
+    ) -> Option<(u128, Scalar, Scalar)> {
         assert!(count > 0);
 
         self.flag.map_or(None, |flag_data| {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_ast/src/token.rs" "/checkout/compiler/rustc_llvm/src/lib.rs" "/checkout/compiler/rustc_llvm/build.rs" "/checkout/compiler/rustc_ast/src/util/comments.rs" "/checkout/compiler/rustc_traits/src/lib.rs" "/checkout/compiler/rustc_target/src/json.rs" "/checkout/compiler/rustc_target/src/abi/mod.rs" "/checkout/compiler/rustc_ast/src/entry.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
