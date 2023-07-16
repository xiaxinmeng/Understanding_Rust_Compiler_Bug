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
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/sys/unix/process/process_fuchsia.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Diff in /checkout/library/std/src/sys/unix/process/process_fuchsia.rs at line 254:
     // other things from std::os::unix) properly.  This veneer is always going to be a bdoge.  So
     // while I don't know if these implementations are actually correct, I think they will do for
     // now at least.
-    pub fn core_dumped(&self) -> bool { false }
-    pub fn stopped_signal(&self) -> Option<i32> { None }
-    pub fn continued(&self) -> bool { false }
+    pub fn core_dumped(&self) -> bool {
+    }
+    }
+    pub fn stopped_signal(&self) -> Option<i32> {
+        None
+    }
+    pub fn continued(&self) -> bool {
+    }
 
 
     pub fn into_raw(&self) -> c_int {
         // We don't know what someone who calls into_raw() will do with this value, but it should
Build completed unsuccessfully in 0:00:18
