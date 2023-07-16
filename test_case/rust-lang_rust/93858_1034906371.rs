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
Diff in /checkout/library/std/src/os/unix/process.rs at line 153:
     /// Sets the process group ID of the child process. Translates to a `setpgid` call in the child
     /// process.
     #[unstable(feature = "process_set_process_group", issue = "93857")]
-    fn process_group(
-        &mut self,
-        pgroup: i32,
-    ) -> &mut process::Command;
+    fn process_group(&mut self, pgroup: i32) -> &mut process::Command;
 
 
 #[stable(feature = "rust1", since = "1.0.0")]
Diff in /checkout/library/std/src/os/unix/process.rs at line 210:
     }
 
-    fn process_group(
-        &mut self,
-        &mut self,
-        pgroup: i32,
-    ) -> &mut process::Command {
+    fn process_group(&mut self, pgroup: i32) -> &mut process::Command {
         self.as_inner_mut().pgroup(pgroup);
     }
     }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/os/unix/ffi/os_str.rs" "/checkout/library/std/src/os/unix/raw.rs" "/checkout/library/std/src/os/unix/ffi/mod.rs" "/checkout/library/std/src/os/unix/mod.rs" "/checkout/library/std/src/os/unix/process.rs" "/checkout/library/std/src/os/unix/fs.rs" "/checkout/library/std/src/os/unix/io/fd/tests.rs" "/checkout/library/std/src/os/illumos/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
