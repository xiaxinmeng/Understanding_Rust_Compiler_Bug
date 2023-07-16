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
Diff in /checkout/src/tools/jsondocck/src/main.rs at line 182:
         commands.push(Command { negated, kind: cmd, args, lineno })
 
-    if !errors {
-        Ok(commands)
-    } else {
-    } else {
-        Err(())
-    }
+    if !errors { Ok(commands) } else { Err(()) }
 
 
 /// Performs the actual work of ensuring a command passes. Generally assumes the command
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/tools/linkchecker/main.rs" "/checkout/src/tools/error_index_generator/build.rs" "/checkout/src/tools/error_index_generator/main.rs" "/checkout/src/tools/jsondocck/src/config.rs" "/checkout/src/tools/jsondocck/src/error.rs" "/checkout/src/tools/jsondocck/src/cache.rs" "/checkout/src/tools/jsondocck/src/main.rs" "/checkout/src/tools/tidy/src/unit_tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
