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
Diff in /checkout/library/alloc/src/collections/vec_deque/mod.rs at line 781:
     pub fn shrink_to(&mut self, min_capacity: usize) {
         // We don't have to worry about an overflow as neither `self.len()` nor `self.capacity()` can ever be `usize::MAX`:
         let min_capacity = cmp::min(min_capacity, self.capacity());
-        let target_cap = cmp::max(
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/alloc/src/collections/vec_deque/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
-            cmp::max(min_capacity, self.len()) + 1,
-            MINIMUM_CAPACITY + 1,
-        )
-        .next_power_of_two();
+        let target_cap = cmp::max(cmp::max(min_capacity, self.len()) + 1, MINIMUM_CAPACITY + 1)
+            .next_power_of_two();
 
         if target_cap < self.cap() {
             // There are three cases of interest:
Build completed unsuccessfully in 0:00:20
