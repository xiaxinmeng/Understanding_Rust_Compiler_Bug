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
Diff in /checkout/src/librustdoc/json/mod.rs at line 108:
                                 .last()
                                 .map(Clone::clone),
                             visibility: types::Visibility::Public,
-                            inner: types::ItemEnum::Trait(
-                                trait_item.clone().into_tcx(self.tcx),
-                            ),
+                            inner: types::ItemEnum::Trait(trait_item.clone().into_tcx(self.tcx)),
                             source: None,
                             docs: Default::default(),
                             links: Default::default(),
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/error.rs" "/checkout/src/librustdoc/json/mod.rs" "/checkout/src/librustdoc/config.rs" "/checkout/src/librustdoc/visit_lib.rs" "/checkout/src/librustdoc/visit_ast.rs" "/checkout/src/librustdoc/clean/utils.rs" "/checkout/src/librustdoc/lib.rs" "/checkout/src/librustdoc/doctest/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:18
