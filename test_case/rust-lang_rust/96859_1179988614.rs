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

######################################################################## 100.0%
extracting /checkout/obj/build/cache/2022-06-29/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz to /checkout/obj/build/x86_64-unknown-linux-gnu/stage0
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/src/bootstrap/compile.rs at line 1118:
         // panic (see #90224). In practice, that's probably ok: nothing currently uses the stage 2
         // sysroot.
         if run.builder.top_stage >= 2 && !run.builder.config.full_bootstrap {
-            let build_compiler = run.builder.compiler(run.builder.top_stage, run.builder.config.build);
+            let build_compiler =
+                run.builder.compiler(run.builder.top_stage, run.builder.config.build);
             run.builder.ensure(Rustc {
                 target: run.target,
                 target: run.target,
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/tarball.rs" "/checkout/src/bootstrap/cc_detect.rs" "/checkout/src/bootstrap/native.rs" "/checkout/src/bootstrap/tool.rs" "/checkout/src/bootstrap/dist.rs" "/checkout/src/bootstrap/compile.rs" "/checkout/src/bootstrap/bin/rustdoc.rs" "/checkout/src/bootstrap/toolstate.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
thread '<unnamed>' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channel', format.rs:166:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread '<unnamed>' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channel', format.rs:166:17
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', /cargo/registry/src/github.com-1ecc6299db9ec823/ignore-0.4.18/src/walk.rs:1302:31
