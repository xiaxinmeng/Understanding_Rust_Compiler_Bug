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
Diff in /checkout/library/alloc/src/collections/vec_deque/mod.rs at line 288:
     unsafe fn wrap_copy(&self, dst: usize, src: usize, len: usize) {
         #[allow(dead_code)]
         fn diff(a: usize, b: usize) -> usize {
-            if a <= b {
-                b - a
-                a - b
-            }
-            }
+            if a <= b { b - a } else { a - b }
         debug_assert!(
         debug_assert!(
             cmp::min(diff(dst, src), self.cap() - diff(dst, src)) + len <= self.cap(),
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/alloc/src/collections/vec_deque/macros.rs" "/checkout/library/alloc/src/collections/btree/node/tests.rs" "/checkout/library/alloc/src/collections/btree/node.rs" "/checkout/library/alloc/src/collections/btree/search.rs" "/checkout/library/alloc/src/collections/btree/dedup_sorted_iter.rs" "/checkout/library/alloc/src/collections/vec_deque/drain.rs" "/checkout/library/alloc/src/collections/vec_deque/mod.rs" "/checkout/library/alloc/src/collections/btree/mem.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
