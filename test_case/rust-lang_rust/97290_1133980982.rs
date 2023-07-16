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
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/src/bootstrap/lib.rs at line 556:
         }
 
         // check_submodule
-        let checked_out_hash = output(
-            Command::new("git").args(&["rev-parse", "HEAD"]).current_dir(&absolute_path),
+        let checked_out_hash =
+        let checked_out_hash =
+            output(Command::new("git").args(&["rev-parse", "HEAD"]).current_dir(&absolute_path));
         // update_submodules
         let recorded = output(
             Command::new("git")
Diff in /checkout/src/bootstrap/config.rs at line 846:
         set(&mut config.docs_minification, build.docs_minification);
         set(&mut config.docs, build.docs);
         if build.fast_submodules.is_some() {
-            println!("warning: the `fast_submodules` option is deprecated and does nothing. Fast submodules are enabled unconditionally.");
+            println!(
+                "warning: the `fast_submodules` option is deprecated and does nothing. Fast submodules are enabled unconditionally."
         }
         }
         set(&mut config.locked_deps, build.locked_deps);
         set(&mut config.vendor, build.vendor);
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/bin/rustdoc.rs" "/checkout/src/bootstrap/builder.rs" "/checkout/src/bootstrap/lib.rs" "/checkout/src/bootstrap/channel.rs" "/checkout/src/bootstrap/test.rs" "/checkout/src/librustdoc/markdown.rs" "/checkout/src/bootstrap/doc.rs" "/checkout/src/bootstrap/build.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
