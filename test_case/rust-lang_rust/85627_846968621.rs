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
Diff in /checkout/compiler/rustc_mir_build/src/check_unsafety.rs at line 124:
             let mut db = lint.build(msg);
             db.span_label(block_span, msg);
             if let Some((span, kind)) = enclosing_unsafe {
-                db.span_label(
-                    span,
-                    format!("because it's nested under this `unsafe` {}", kind),
-                );
+                db.span_label(span, format!("because it's nested under this `unsafe` {}", kind));
             }
             db.emit();
         });
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir_build/src/lints.rs" "/checkout/library/proc_macro/src/diagnostic.rs" "/checkout/compiler/rustc_mir_build/src/check_unsafety.rs" "/checkout/library/proc_macro/src/quote.rs" "/checkout/library/profiler_builtins/build.rs" "/checkout/compiler/rustc_mir_build/src/thir/visit.rs" "/checkout/compiler/rustc_mir_build/src/thir/cx/mod.rs" "/checkout/compiler/rustc_mir_build/src/build/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
