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
Diff in /checkout/src/bootstrap/cc_detect.rs at line 181:
 
                 // API 19 is the earliest API level supported by NDK r25b but AArch64 and x86_64 support
                 // begins at API level 21.
-                let api_level = if t.contains("aarch64") || t.contains("x86_64") { "21" } else { "19" };
+                let api_level =
+                    if t.contains("aarch64") || t.contains("x86_64") { "21" } else { "19" };
                 let compiler = format!("{}{}-{}", triple_translated, api_level, compiler.clang());
                 cfg.compiler(ndk.join("bin").join(compiler));
             }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/cc_detect.rs" "/checkout/src/bootstrap/format.rs" "/checkout/src/bootstrap/compile.rs" "/checkout/src/bootstrap/channel.rs" "/checkout/library/alloc/src/vec/in_place_collect.rs" "/checkout/library/alloc/src/vec/splice.rs" "/checkout/library/alloc/src/vec/in_place_drop.rs" "/checkout/src/bootstrap/toolstate.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
