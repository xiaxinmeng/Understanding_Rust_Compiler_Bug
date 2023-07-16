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
Diff in /checkout/compiler/rustc_expand/src/mbe/macro_parser.rs at line 630:
 
             // Process `cur_mps` until either we have finished the input or we need to get some
             // parsing from the black-box parser done.
-            let res = self.parse_tt_inner(matcher, &parser.token, parser.approx_token_stream_pos(), track);
+            let res = self.parse_tt_inner(
+                &parser.token,
+                &parser.token,
+                parser.approx_token_stream_pos(),
+                track,
             if let Some(res) = res {
                 return res;
             }
             }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/tools/expand-yaml-anchors/src/main.rs" "/checkout/compiler/rustc_expand/src/mbe/macro_parser.rs" "/checkout/compiler/rustc_expand/src/mbe/macro_check.rs" "/checkout/compiler/rustc_expand/src/mbe/macro_rules.rs" "/checkout/compiler/rustc_expand/src/mbe/metavar_expr.rs" "/checkout/compiler/rustc_expand/src/mbe/transcribe.rs" "/checkout/compiler/rustc_expand/src/mbe/quoted.rs" "/checkout/compiler/rustc_expand/src/mbe.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
