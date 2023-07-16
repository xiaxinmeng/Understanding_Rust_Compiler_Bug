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
Diff in /checkout/src/bootstrap/config.rs at line 895:
             config.rust_codegen_units_std = rust.codegen_units_std.map(threads_from_config);
             config.rust_profile_use = flags.rust_profile_use.or(rust.profile_use);
             config.rust_profile_generate = flags.rust_profile_generate.or(rust.profile_generate);
-            config.rust_profile_sample_use = flags.rust_profile_sample_use.or(rust.profile_sample_use);
+            config.rust_profile_sample_use =
+                flags.rust_profile_sample_use.or(rust.profile_sample_use);
             config.download_rustc = env::var("BOOTSTRAP_DOWNLOAD_RUSTC").as_deref() == Ok("1");
         } else {
             config.rust_profile_use = flags.rust_profile_use;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/config.rs" "/checkout/src/bootstrap/bin/rustc.rs" "/checkout/compiler/rustc_ast_pretty/src/pprust/state.rs" "/checkout/compiler/rustc_hir/src/tests.rs" "/checkout/compiler/rustc_ast_pretty/src/pprust/mod.rs" "/checkout/compiler/rustc_hir/src/intravisit.rs" "/checkout/compiler/rustc_ast_pretty/src/pprust/tests.rs" "/checkout/src/bootstrap/format.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
