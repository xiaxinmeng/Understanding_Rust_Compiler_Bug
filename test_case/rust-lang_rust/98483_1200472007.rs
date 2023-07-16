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
######################                                                    31.0%
######################################################################## 100.0%
extracting /checkout/obj/build/cache/2022-07-21/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz to /checkout/obj/build/x86_64-unknown-linux-gnu/stage0
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/src/bootstrap/dist.rs at line 1910:
         let mut cmd = Command::new(llvm_config);
         cmd.arg("--libfiles");
         builder.verbose(&format!("running {:?}", cmd));
-        let files = if builder.config.dry_run {
-            "".into()
-            output(&mut cmd)
-        };
-        };
+        let files = if builder.config.dry_run { "".into() } else { output(&mut cmd) };
         let build_llvm_out = &builder.llvm_out(builder.config.build);
         let target_llvm_out = &builder.llvm_out(target);
         for file in files.trim_end().split(' ') {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/run.rs" "/checkout/src/bootstrap/util.rs" "/checkout/src/bootstrap/builder.rs" "/checkout/src/bootstrap/tool.rs" "/checkout/src/bootstrap/dist.rs" "/checkout/src/bootstrap/config.rs" "/checkout/src/bootstrap/format.rs" "/checkout/compiler/rustc_codegen_llvm/src/debuginfo/create_scope_map.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
thread 'thread '<unnamed>' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channel<unnamed>', ' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channelformat.rs', :format.rs166::166:1717
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', /cargo/registry/src/github.com-1ecc6299db9ec823/ignore-0.4.18/src/walk.rs:1302:31
Build completed unsuccessfully in 0:00:16
