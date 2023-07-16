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
Diff in /checkout/src/bootstrap/dist.rs at line 1233:
     }
 
     fn make_run(run: RunConfig<'_>) {
-        run.builder.ensure(RustDemangler {
-            target: run.target,
-        });
+        run.builder.ensure(RustDemangler { target: run.target });
 
 
     fn run(self, builder: &Builder<'_>) -> Option<GeneratedTarball> {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/dist.rs" "/checkout/library/proc_macro/src/bridge/rpc.rs" "/checkout/src/bootstrap/config.rs" "/checkout/src/bootstrap/tarball.rs" "/checkout/src/bootstrap/util.rs" "/checkout/src/librustdoc/theme.rs" "/checkout/src/bootstrap/builder/tests.rs" "/checkout/library/proc_macro/src/bridge/server.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
