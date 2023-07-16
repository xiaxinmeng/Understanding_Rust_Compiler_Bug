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
Diff in /checkout/src/tools/jsondoclint/src/validator.rs at line 304:
             rustdoc_json_types::GenericParamDefKind::Const { type_, default: _ } => {
                 self.check_type(type_)
             }
-            rustdoc_json_types::GenericParamDefKind::Effect { } => {}
+            rustdoc_json_types::GenericParamDefKind::Effect {} => {}
     }
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/tools/bump-stage0/src/main.rs" "/checkout/src/tools/lld-wrapper/src/main.rs" "/checkout/src/tools/remote-test-client/src/main.rs" "/checkout/src/tools/html-checker/main.rs" "/checkout/src/tools/jsondoclint/src/item_kind.rs" "/checkout/src/tools/jsondoclint/src/main.rs" "/checkout/src/tools/jsondoclint/src/validator.rs" "/checkout/src/tools/rust-demangler/src/lib.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
