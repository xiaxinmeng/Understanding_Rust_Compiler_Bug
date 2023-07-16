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
extracting /checkout/obj/build/cache/2022-07-21/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz to /checkout/obj/build/x86_64-unknown-linux-gnu/stage0
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/src/bootstrap/dist.rs at line 362:
 
             builder.install(&builder.rustdoc(compiler), &image.join("bin"), 0o755);
 
-            let ra_proc_macro_srv = builder.ensure(tool::RustAnalyzerProcMacroSrv {
-                target: compiler.host,
-                target: compiler.host,
-            }).expect("rust-analyzer-proc-macro-server always builds");
+            let ra_proc_macro_srv = builder
+                .ensure(tool::RustAnalyzerProcMacroSrv { compiler, target: compiler.host })
+                .expect("rust-analyzer-proc-macro-server always builds");
             builder.install(&ra_proc_macro_srv, &image.join("libexec"), 0o755);
             let libdir_relative = builder.libdir_relative(compiler);
             let libdir_relative = builder.libdir_relative(compiler);
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/native.rs" "/checkout/src/bootstrap/bin/sccache-plus-cl.rs" "/checkout/src/bootstrap/dist.rs" "/checkout/src/bootstrap/bin/llvm-config-wrapper.rs" "/checkout/src/bootstrap/config.rs" "/checkout/src/bootstrap/bin/rustc.rs" "/checkout/src/bootstrap/format.rs" "/checkout/library/proc_macro/src/bridge/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
thread '<unnamed>' panicked at 'thread '<unnamed>' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channel', format.rs:166:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
tx.send(entry.into_path()) failed with sending on a closed channel', format.rs:166:17
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', /cargo/registry/src/github.com-1ecc6299db9ec823/ignore-0.4.18/src/walk.rs:1302:31
