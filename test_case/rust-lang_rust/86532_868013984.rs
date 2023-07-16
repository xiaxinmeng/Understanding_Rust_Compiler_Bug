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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_errors/src/emitter.rs at line 1706:
                         );
                         // If this is a replacement, underline with `^`, if this is an addition
                         // underline with `+`.
-                        buffer.putc(
-                            row_num,
-                            (padding as isize + p) as usize,
-                            '^',
-                            Style::Addition,
-                        );
+                        buffer.putc(row_num, (padding as isize + p) as usize, '^', Style::Addition);
                     if show_diff {
                     if show_diff {
                         // Colorize removal with red in diff format.
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_infer/src/lib.rs" "/checkout/compiler/rustc_infer/src/traits/error_reporting/mod.rs" "/checkout/compiler/rustc_errors/src/snippet.rs" "/checkout/compiler/rustc_infer/src/infer/lattice.rs" "/checkout/compiler/rustc_errors/src/json/tests.rs" "/checkout/compiler/rustc_errors/src/diagnostic_builder.rs" "/checkout/compiler/rustc_errors/src/emitter.rs" "/checkout/compiler/rustc_errors/src/lock.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:13
