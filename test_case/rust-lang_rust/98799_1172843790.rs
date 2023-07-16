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
########                                                                  12.2%
######################################################################## 100.0%
extracting /checkout/obj/build/cache/2022-05-20/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz to /checkout/obj/build/x86_64-unknown-linux-gnu/stage0
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/src/librustdoc/lib.rs at line 778:
                 sess.opts.debugging_opts.no_interleave_lints,
                 sess.unstable_options(),
-            let registered_lints =
-                if let Some(register_lints) = compiler.register_lints() {
-                if let Some(register_lints) = compiler.register_lints() {
-                    register_lints(sess, &mut lint_store);
-                } else {
-                    false
-                };
+            let registered_lints = if let Some(register_lints) = compiler.register_lints() {
+            let registered_lints = if let Some(register_lints) = compiler.register_lints() {
+                register_lints(sess, &mut lint_store);
+            } else {
+                false
+            };
+            };
             rustc_driver::describe_lints(sess, &lint_store, registered_lints);
             return Ok(());
         }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/lib.rs" "/checkout/src/librustdoc/doctest.rs" "/checkout/src/librustdoc/docfs.rs" "/checkout/src/librustdoc/visit_lib.rs" "/checkout/src/librustdoc/markdown.rs" "/checkout/src/bootstrap/dist.rs" "/checkout/src/bootstrap/compile.rs" "/checkout/src/librustdoc/visit_ast.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
