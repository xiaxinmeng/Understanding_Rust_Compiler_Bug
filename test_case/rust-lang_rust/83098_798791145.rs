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
Diff in /checkout/compiler/rustc_passes/src/check_attr.rs at line 559:
                                 |lint| {
                                     lint.build(
                                         "`#![doc(test(...)]` is only allowed \
-                                         as a crate level attribute"
+                                         as a crate level attribute",
                                     )
                                     .emit();
                                 },
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_passes/src/dead.rs" "/checkout/compiler/rustc_lexer/src/unescape/tests.rs" "/checkout/compiler/rustc_passes/src/stability.rs" "/checkout/compiler/rustc_passes/src/lang_items.rs" "/checkout/compiler/rustc_passes/src/hir_stats.rs" "/checkout/compiler/rustc_lexer/src/unescape.rs" "/checkout/compiler/rustc_passes/src/check_attr.rs" "/checkout/compiler/rustc_lexer/src/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
