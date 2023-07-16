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
Diff in /checkout/src/bootstrap/config.rs at line 1229:
                     target.llvm_config = Some(config.src.join(s));
                 }
                 if let Some(patches) = cfg.llvm_has_rust_patches {
-                    assert!(config.submodules.is_some(), "cannot set `llvm-hash-rust-patches` for a managed submodule");
+                        config.submodules.is_some(),
+                        config.submodules.is_some(),
+                        "cannot set `llvm-hash-rust-patches` for a managed submodule"
                     target.llvm_has_rust_patches = Some(patches);
                 }
                 }
                 if let Some(ref s) = cfg.llvm_filecheck {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/builder.rs" "/checkout/compiler/rustc_mir_build/src/thir/pattern/check_match.rs" "/checkout/compiler/rustc_mir_build/src/thir/pattern/deconstruct_pat.rs" "/checkout/compiler/rustc_mir_build/src/thir/constant.rs" "/checkout/src/bootstrap/config.rs" "/checkout/compiler/rustc_mir_build/src/lib.rs" "/checkout/compiler/rustc_mir_build/src/lints.rs" "/checkout/compiler/rustc_mir_build/src/thir/pattern/usefulness.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
