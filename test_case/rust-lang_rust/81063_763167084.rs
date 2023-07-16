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
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/tools/jsondocck/src/main.rs"` failed.
Diff in /checkout/src/tools/jsondocck/src/main.rs at line 149:
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
         }
 
 
-        let args = cap.name("args")
-            .map_or(vec![], |m| shlex::split(m.as_str()).unwrap());
+        let args = cap.name("args").map_or(vec![], |m| shlex::split(m.as_str()).unwrap());
 
         if !cmd.validate(&args, commands.len(), lineno) {
             errors = true;
Build completed unsuccessfully in 0:00:22
