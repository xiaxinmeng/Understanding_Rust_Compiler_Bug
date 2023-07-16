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
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/src/librustdoc/clean/types.rs at line 1034:
                     DocFragmentKind::RawDoc
 
 
-                let frag = DocFragment {
-                    span: attr.span,
-                    doc: value,
-                    parent_module,
-                    indent: 0,
-                };
+                let frag =
+                let frag =
+                    DocFragment { span: attr.span, doc: value, kind, parent_module, indent: 0 };
 
                 doc_strings.push(frag);
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/clean/types.rs" "/checkout/src/librustdoc/clean/cfg/tests.rs" "/checkout/src/tools/tidy/src/features/version.rs" "/checkout/src/tools/tidy/src/lib.rs" "/checkout/src/librustdoc/clean/utils.rs" "/checkout/src/tools/tidy/src/errors.rs" "/checkout/src/tools/tidy/src/debug_artifacts.rs" "/checkout/src/tools/tidy/src/features.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
