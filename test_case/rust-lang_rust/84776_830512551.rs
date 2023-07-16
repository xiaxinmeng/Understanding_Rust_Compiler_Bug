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
Diff in /checkout/src/bootstrap/tool.rs at line 400:
         crate::dist::maybe_install_llvm_runtime(builder, compiler.host, &sysroot);
         add_dylib_path(
-            vec![
-            vec![
-                builder.sysroot_libdir(compiler, compiler.host).to_path_buf(),
-            ],
-            ],
+            vec![builder.sysroot_libdir(compiler, compiler.host).to_path_buf(), rustc_libdir],
         );
         cmd
         cmd
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/tool.rs" "/checkout/src/bootstrap/cc_detect.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:13
