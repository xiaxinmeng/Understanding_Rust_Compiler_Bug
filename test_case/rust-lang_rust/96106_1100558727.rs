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
Diff in /checkout/src/bootstrap/builder.rs at line 1:
-use std::any::{Any, type_name};
+use std::any::{type_name, Any};
 use std::cell::{Cell, RefCell};
 use std::collections::BTreeSet;
 use std::env;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/builder/tests.rs" "/checkout/src/bootstrap/metadata.rs" "/checkout/src/bootstrap/util.rs" "/checkout/src/bootstrap/lib.rs" "/checkout/src/librustdoc/externalfiles.rs" "/checkout/src/bootstrap/dist.rs" "/checkout/src/librustdoc/lib.rs" "/checkout/src/bootstrap/builder.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Diff in /checkout/src/bootstrap/builder.rs at line 1730:
 
 
         if self.config.print_step_timings && !self.config.dry_run {
-            println!("[TIMING] ({}) {:?} -- {}.{:03}", type_name::<S>(), step, dur.as_secs(), dur.subsec_millis());
+            println!(
+                "[TIMING] ({}) {:?} -- {}.{:03}",
+                type_name::<S>(),
+                step,
+                dur.as_secs(),
+                dur.subsec_millis()
         }
 
         {
Build completed unsuccessfully in 0:00:13
