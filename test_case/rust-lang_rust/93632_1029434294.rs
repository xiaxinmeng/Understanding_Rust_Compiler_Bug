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
Diff in /checkout/src/librustdoc/clean/mod.rs at line 1533:
                 for pb in obj.projection_bounds() {
                     bindings.push(TypeBinding {
                         name: cx.tcx.associated_item(pb.item_def_id()).name,
-                        kind: TypeBindingKind::Equality {
-                            term: pb.skip_binder().term.clean(cx),
-                        },
+                        kind: TypeBindingKind::Equality { term: pb.skip_binder().term.clean(cx) },
                 }
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/visit.rs" "/checkout/src/librustdoc/clean/utils/tests.rs" "/checkout/src/librustdoc/theme.rs" "/checkout/src/librustdoc/docfs.rs" "/checkout/src/librustdoc/doctest.rs" "/checkout/src/librustdoc/clean/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
