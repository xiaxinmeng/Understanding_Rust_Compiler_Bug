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
Diff in /checkout/compiler/rustc_codegen_ssa/src/back/link.rs at line 889:
 fn link_sanitizer_runtime(sess: &Session, linker: &mut dyn Linker, name: &str) {
     fn find_sanitizer_runtime(sess: &Session, filename: &String) -> PathBuf {
         let session_tlib =
-            filesearch::make_target_lib_path(&sess.sysroot,
-                sess.opts.target_triple.triple());
+            filesearch::make_target_lib_path(&sess.sysroot, sess.opts.target_triple.triple());
         let path = session_tlib.join(&filename);
         if path.exists() {
             return session_tlib;
Diff in /checkout/compiler/rustc_codegen_ssa/src/back/link.rs at line 897:
         } else {
             let default_sysroot = filesearch::get_or_default_sysroot();
-            let default_tlib =
-                filesearch::make_target_lib_path(&default_sysroot,
-                    sess.opts.target_triple.triple());
+            let default_tlib = filesearch::make_target_lib_path(
+                &default_sysroot,
+                sess.opts.target_triple.triple(),
+            );
             return default_tlib;
     }
     }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_ssa/src/back/link.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
