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
Diff in /checkout/src/bootstrap/test.rs at line 132:
         // docs built for the other platform (e.g. rustc linking to cargo)
         if (hosts != targets) && !hosts.is_empty() && !targets.is_empty() {
             panic!(
-"Linkcheck currently does not support builds with different hosts and targets.
-You can skip linkcheck with --exclude src/tools/linkchecker");
+                "Linkcheck currently does not support builds with different hosts and targets.
+You can skip linkcheck with --exclude src/tools/linkchecker"
+            );
         }
         run.default_condition(builder.config.docs)
     }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/build.rs" "/checkout/src/bootstrap/cache.rs" "/checkout/src/bootstrap/test.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:15
