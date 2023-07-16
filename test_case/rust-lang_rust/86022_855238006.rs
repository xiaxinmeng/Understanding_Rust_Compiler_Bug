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
Diff in /checkout/src/bootstrap/builder.rs at line 3:
 use std::collections::BTreeSet;
 use std::env;
 use std::ffi::OsStr;
-use std::fmt::{self, Display, Debug};
+use std::fmt::{self, Debug, Display};
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc/src/main.rs" "/checkout/src/bootstrap/metadata.rs" "/checkout/compiler/rustc_index/src/bit_set/tests.rs" "/checkout/compiler/rustc_index/src/bit_set.rs" "/checkout/compiler/rustc_index/src/lib.rs" "/checkout/compiler/rustc_index/src/vec/tests.rs" "/checkout/compiler/rustc_index/src/vec.rs" "/checkout/src/bootstrap/builder.rs"` failed.
 use std::fs;
 use std::hash::Hash;
Diff in /checkout/src/bootstrap/builder.rs at line 1567:
Diff in /checkout/src/bootstrap/builder.rs at line 1567:
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
                 path: path.expect("no paths for step"),
             };
             // NOTE: don't hold onto this guard, it will cause a deadlock if the current step calls `ensure` recursively.
-            let old_instructions = CURRENT_INSTRUCTIONS.lock().expect("steps are not run in parallel").replace(instructions);
+            let old_instructions = CURRENT_INSTRUCTIONS
+                .lock()
+                .expect("steps are not run in parallel")
+                .replace(instructions);
             let start = Instant::now();
             let start = Instant::now();
             let zero = Duration::new(0, 0);
Diff in /checkout/src/bootstrap/builder.rs at line 1607:
 
 
 pub(crate) extern "C" fn print_replication_steps() {
-    if let Some(step) = CURRENT_INSTRUCTIONS.lock().expect("mutex guard is dropped on panic").take() {
+    if let Some(step) = CURRENT_INSTRUCTIONS.lock().expect("mutex guard is dropped on panic").take()
+    {
         println!("note: failed while building {}", step.name);
-        println!("help: to replicate this failure, run `./x.py {} {}`", step.cmd, step.path.display());
+        println!(
+            "help: to replicate this failure, run `./x.py {} {}`",
+            step.cmd,
+            step.path.display()
     }
 }
 
note: failed while building bootstrap::test::Tidy
note: failed while building bootstrap::test::Tidy
help: to replicate this failure, run `./x.py test src/tools/tidy`
Build completed unsuccessfully in 0:00:19
