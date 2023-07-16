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
Diff in /checkout/compiler/rustc_infer/src/infer/error_reporting/mod.rs at line 2061:
                     found: exp_found.found.print_only_trait_path(),
                 };
                 match self.expected_found_str(pretty_exp_found) {
-                    Some((expected, found)) if expected == found => self.expected_found_str(exp_found),
+                    Some((expected, found)) if expected == found => {
+                        self.expected_found_str(exp_found)
                     ret => ret,
                 }
             }
             }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_infer/src/infer/opaque_types.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs" "/checkout/compiler/rustc_infer/src/infer/combine.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/mod.rs" "/checkout/compiler/rustc_infer/src/infer/equate.rs" "/checkout/compiler/rustc_infer/src/infer/lub.rs" "/checkout/compiler/rustc_infer/src/infer/undo_log.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/note.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
