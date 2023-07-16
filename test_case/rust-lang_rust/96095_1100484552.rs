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
Diff in /checkout/src/librustdoc/formats/cache.rs at line 476:
             } else {
                 let trait_did = impl_item.trait_did().expect("no trait did");
                 if let ItemId::Blanket { for_, .. } = &impl_item.impl_item.def_id {
-                    dids = dids
-                        .into_iter()
-                        .filter(|did| did == for_)
-                        .collect();
+                    dids = dids.into_iter().filter(|did| did == for_).collect();
                 }
                 self.cache.orphan_trait_impls.push((trait_did, dids, impl_item));
             }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/toolstate.rs" "/checkout/src/bootstrap/cache.rs" "/checkout/src/librustdoc/formats/item_type.rs" "/checkout/src/librustdoc/formats/mod.rs" "/checkout/library/rustc-std-workspace-alloc/lib.rs" "/checkout/compiler/rustc_fs_util/src/lib.rs" "/checkout/compiler/rustc_lint/src/non_fmt_panic.rs" "/checkout/src/librustdoc/formats/cache.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
