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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_middle/src/ty/layout.rs at line 1164:
 
                         let abi = match variant_layouts[largest_variant_index].abi {
                             Abi::Scalar(_) if others_zst => Abi::Scalar(niche_scalar),
-                            Abi::ScalarPair(first, second) if others_zst => if niche_offset == Size::ZERO {
-                                Abi::ScalarPair(niche_scalar, scalar_unit(second.value))
-                            } else {
-                                Abi::ScalarPair(scalar_unit(first.value), niche_scalar)
-                            },
-                            _ => Abi::Aggregate { sized: true }
+                            Abi::ScalarPair(first, second) if others_zst => {
+                                if niche_offset == Size::ZERO {
+                                    Abi::ScalarPair(niche_scalar, scalar_unit(second.value))
+                                } else {
+                                    Abi::ScalarPair(scalar_unit(first.value), niche_scalar)
+                            }
+                            }
+                            _ => Abi::Aggregate { sized: true },
 
                         Ok(Some(Layout {
                         Ok(Some(Layout {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_middle/src/hir/mod.rs" "/checkout/compiler/rustc_middle/src/hir/nested_filter.rs" "/checkout/compiler/rustc_middle/src/hir/place.rs" "/checkout/compiler/rustc_middle/src/util/common.rs" "/checkout/compiler/rustc_middle/src/ty/assoc.rs" "/checkout/compiler/rustc_middle/src/util/common/tests.rs" "/checkout/compiler/rustc_middle/src/ty/fast_reject.rs" "/checkout/compiler/rustc_middle/src/ty/layout.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
