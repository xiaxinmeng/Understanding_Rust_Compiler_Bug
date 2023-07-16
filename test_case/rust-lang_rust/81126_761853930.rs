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
Diff in /checkout/library/alloc/src/vec/mod.rs at line 1411:
             }
         }
 
-        let mut guard = BackshiftOnDrop {
-            v: self,
-            processed_len: 0,
-            deleted_cnt: 0,
-            original_len: len,
+        let mut guard =
+        let mut guard =
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/alloc/src/vec/mod.rs"` failed.
+            BackshiftOnDrop { v: self, processed_len: 0, deleted_cnt: 0, original_len: len };
 
         let mut del = 0usize;
         for i in 0..len {
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:17
