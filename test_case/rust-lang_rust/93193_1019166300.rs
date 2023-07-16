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
Diff in /checkout/compiler/rustc_data_structures/src/stable_hasher/tests.rs at line 103:
 #[test]
     macro_rules! test_type {
     macro_rules! test_type {
-        ($ty: ty) => {
-                struct Foo {
-                struct Foo {
-                    a: $ty,
-                    b: $ty
-                }
+        ($ty: ty) => {{
+            struct Foo {
+                a: $ty,
+                b: $ty,
 
 
-                impl<CTX> HashStable<CTX> for Foo {
-                    fn hash_stable(&self, hcx: &mut CTX, hasher: &mut StableHasher) {
-                        self.a.hash_stable(hcx, hasher);
-                        self.b.hash_stable(hcx, hasher);
-                    }
+            impl<CTX> HashStable<CTX> for Foo {
+                fn hash_stable(&self, hcx: &mut CTX, hasher: &mut StableHasher) {
+                    self.a.hash_stable(hcx, hasher);
+                    self.b.hash_stable(hcx, hasher);
-
-                #[allow(overflowing_literals)]
-                #[allow(overflowing_literals)]
-                let mut item = Foo { a: 0xFF, b: 0xFF_FF };
-                let hash_a = hash(&item);
-                std::mem::swap(&mut item.a, &mut item.b);
-                let hash_b = hash(&item);
-                assert_ne!(
-                    hash_a,
-                    hash_b,
-                    "The hash stayed the same after values were swapped for type `{}`!", stringify!($ty)
             }
-        };
+
+            #[allow(overflowing_literals)]
+            #[allow(overflowing_literals)]
+            let mut item = Foo { a: 0xFF, b: 0xFF_FF };
+            let hash_a = hash(&item);
+            std::mem::swap(&mut item.a, &mut item.b);
+            let hash_b = hash(&item);
+            assert_ne!(
+                hash_a,
+                hash_b,
+                "The hash stayed the same after values were swapped for type `{}`!",
+                stringify!($ty)
+        }};
     }
 
 
     test_type!(u16);
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/unwind/src/lib.rs" "/checkout/compiler/rustc_data_structures/src/stable_hasher/tests.rs" "/checkout/compiler/rustc_data_structures/src/profiling.rs" "/checkout/library/rtstartup/rsend.rs" "/checkout/library/rtstartup/rsbegin.rs" "/checkout/library/panic_unwind/src/emcc.rs" "/checkout/compiler/rustc_data_structures/src/vec_map.rs" "/checkout/library/std/src/lib.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
