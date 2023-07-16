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
Diff in /checkout/compiler/rustc_codegen_ssa/src/back/link.rs at line 2187:
 
     // Converts a library file-stem into a cc -l argument
     fn unlib<'a>(target: &Target, stem: &'a str) -> &'a str {
-        if stem.starts_with("lib") && !target.is_like_windows {
-            &stem[3..]
-            stem
-        }
-        }
+        if stem.starts_with("lib") && !target.is_like_windows { &stem[3..] } else { stem }
 
 
     // Adds the static "rlib" versions of all crates to the command line.
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_ssa/src/back/rpath/tests.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/metadata.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/link.rs" "/checkout/compiler/rustc_codegen_ssa/src/common.rs" "/checkout/compiler/rustc_lint/src/methods.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/archive.rs" "/checkout/compiler/rustc_lint/src/early.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/command.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:18
