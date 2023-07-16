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
Diff in /checkout/src/bootstrap/compile.rs at line 1159:
         let sysroot_lib_rustlib_rustcsrc = sysroot.join("lib/rustlib/rustc-src");
         t!(fs::create_dir_all(&sysroot_lib_rustlib_rustcsrc));
         let sysroot_lib_rustlib_rustcsrc_rust = sysroot_lib_rustlib_rustcsrc.join("rust");
-        if let Err(e) = symlink_dir(&builder.config, &builder.src, &sysroot_lib_rustlib_rustcsrc_rust) {
+        if let Err(e) =
+            symlink_dir(&builder.config, &builder.src, &sysroot_lib_rustlib_rustcsrc_rust)
             eprintln!(
             eprintln!(
                 "warning: creating symbolic link `{}` to `{}` failed with {}",
                 sysroot_lib_rustlib_rustcsrc_rust.display(),
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/builder/tests.rs" "/checkout/src/bootstrap/compile.rs" "/checkout/src/bootstrap/cache.rs" "/checkout/src/bootstrap/config.rs" "/checkout/src/bootstrap/native.rs" "/checkout/src/bootstrap/flags.rs" "/checkout/src/bootstrap/doc.rs" "/checkout/src/bootstrap/tool.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
