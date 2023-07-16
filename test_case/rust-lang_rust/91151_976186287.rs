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
Diff in /checkout/library/std/src/process/tests.rs at line 56:
 fn signal_reported_right() {
     use crate::os::unix::process::ExitStatusExt;
 
-    let mut p =
-        shell_cmd().arg("-c").arg("read a").stdin(Stdio::piped()).spawn().unwrap();
+    let mut p = shell_cmd().arg("-c").arg("read a").stdin(Stdio::piped()).spawn().unwrap();
     p.kill().unwrap();
     match p.wait().unwrap().signal() {
         Some(9) => {}
Build completed unsuccessfully in 0:00:10
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/os/linux/mod.rs" "/checkout/library/std/src/os/linux/fs.rs" "/checkout/library/std/src/os/ios/raw.rs" "/checkout/library/std/src/fs/tests.rs" "/checkout/library/std/src/sync/rwlock.rs" "/checkout/library/std/src/sync/once/tests.rs" "/checkout/library/std/src/process/tests.rs" "/checkout/library/std/src/os/linux/process.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
