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
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/std/src/sys/unix/process/process_common.rs at line 535:
         debug_command.finish()?;
         write!(f, "\nCommand line: ")?;
         if self.program != self.args[0] {
-             write!(f, "[{:?}] ", self.program)?;
-         }
-         write!(f, "{:?}", self.args[0])?;
+            write!(f, "[{:?}] ", self.program)?;
+        }
+        write!(f, "{:?}", self.args[0])?;
 
-         for arg in &self.args[1..] {
-             write!(f, " {:?}", arg)?;
-         Ok(())
-         Ok(())
+        for arg in &self.args[1..] {
+            write!(f, " {:?}", arg)?;
+        Ok(())
     }
 }
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/sys/unix/process/process_unix/tests.rs" "/checkout/library/std/src/sys/unix/process/mod.rs" "/checkout/library/std/src/sys/unix/process/process_unsupported.rs" "/checkout/library/std/src/sys/unix/process/process_common/tests.rs" "/checkout/library/std/src/sys/unix/process/process_unix.rs" "/checkout/library/std/src/sys/unix/process/process_common.rs" "/checkout/library/std/src/sys/unix/process/process_fuchsia.rs" "/checkout/library/std/src/sys/unix/process/zircon.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
