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
Diff in /checkout/library/alloc/src/boxed.rs at line 1338:
 
     fn clone_from(&mut self, other: &Self) {
         if self.len() == other.len() {
-            unsafe {
-                self.as_bytes_mut().copy_from_slice(other.as_bytes())
-            }
+            unsafe { self.as_bytes_mut().copy_from_slice(other.as_bytes()) }
         } else {
             *self = other.clone();
         }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/alloc/src/fmt.rs" "/checkout/library/alloc/src/tests.rs" "/checkout/library/alloc/src/boxed.rs" "/checkout/library/test/src/types.rs" "/checkout/library/alloc/src/slice.rs" "/checkout/library/test/src/stats/tests.rs" "/checkout/library/test/src/term/win.rs" "/checkout/library/alloc/src/vec/in_place_collect.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
