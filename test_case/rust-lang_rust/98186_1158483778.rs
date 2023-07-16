plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_expand/src/proc_macro_server.rs at line 570:
         punct.ch
     }
     fn spacing(&mut self, punct: Self::Punct) -> Spacing {
-        if punct.joint {
-            Spacing::Joint
-            Spacing::Alone
-        }
-        }
+        if punct.joint { Spacing::Joint } else { Spacing::Alone }
     }
     fn span(&mut self, punct: Self::Punct) -> Self::Span {
         punct.span
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_traits/src/normalize_projection_ty.rs" "/checkout/compiler/rustc_expand/src/mbe/macro_rules.rs" "/checkout/compiler/rustc_expand/src/mbe/transcribe.rs" "/checkout/compiler/rustc_expand/src/mbe/quoted.rs" "/checkout/compiler/rustc_expand/src/mbe/macro_parser.rs" "/checkout/compiler/rustc_expand/src/mbe/macro_check.rs" "/checkout/compiler/rustc_expand/src/proc_macro_server.rs" "/checkout/compiler/rustc_expand/src/mbe/metavar_expr.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
