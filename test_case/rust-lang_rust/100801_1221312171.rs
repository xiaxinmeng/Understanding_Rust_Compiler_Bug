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
Diff in /checkout/compiler/rustc_interface/src/passes.rs at line 599:
             escape_dep_filename(&file.prefer_local().to_string())
 
 
-        let extra_tracked_files = file_depinfo.iter().map(|path_sym| {
-            normalize_path(PathBuf::from(path_sym.as_str()))
+        let extra_tracked_files =
+        let extra_tracked_files =
+            file_depinfo.iter().map(|path_sym| normalize_path(PathBuf::from(path_sym.as_str())));
         files.extend(extra_tracked_files);
 
         // We also need to track used PGO profile files
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_lint/src/nonstandard_style/tests.rs" "/checkout/compiler/rustc_interface/src/passes.rs" "/checkout/compiler/rustc_interface/src/util.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
