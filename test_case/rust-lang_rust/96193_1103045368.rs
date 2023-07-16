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
Diff in /checkout/library/std/src/sys/unix/os.rs at line 461:
     #[cfg(not(test))]
     use crate::env;
 
-    let exe_path = env::args().next()
-        .ok_or(io::const_io_error!(ErrorKind::Uncategorized, "argv[0] did not contain an executable path"))?;
+    let exe_path = env::args().next().ok_or(io::const_io_error!(
+        ErrorKind::Uncategorized,
+        "argv[0] did not contain an executable path"
+    ))?;
     let path = PathBuf::from(exe_path);
 
     // Prepend the current working directory to the path if it's not absolute.
Diff in /checkout/library/std/src/sys/unix/os.rs at line 469:
-    if !path.is_absolute() {
-        getcwd().map(|cwd| cwd.join(path))
-        Ok(path)
-    }
-    }
+    if !path.is_absolute() { getcwd().map(|cwd| cwd.join(path)) } else { Ok(path) }
 
 pub struct Env {
 pub struct Env {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/sys/unix/net.rs" "/checkout/library/std/src/sys/itron/thread.rs" "/checkout/library/std/src/sys/itron/time/tests.rs" "/checkout/library/std/src/sys/unix/android.rs" "/checkout/library/std/src/sys/unix/locks/mod.rs" "/checkout/library/std/src/sys/unix/fd.rs" "/checkout/library/std/src/sys/unix/os.rs" "/checkout/library/std/src/sys/itron/spin.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
