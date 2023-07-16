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
Diff in /checkout/compiler/rustc_mir_transform/src/single_enum.rs at line 29:
                 // Multiple values, cannot reach any of them
                 FlatSet::Top => return None,
 
-                FlatSet::Elem(v) => targets.iter().find(|(tgt_val, _)| tgt_val == &(v.as_u32() as u128))?.1,
+                FlatSet::Elem(v) => {
+                    targets.iter().find(|(tgt_val, _)| tgt_val == &(v.as_u32() as u128))?.1
+                }
                 // Uninhabited (no enums can be reached), so assign to otherwise branch.
                 FlatSet::Bottom => targets.otherwise(),
             };
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir_transform/src/normalize_array_len.rs" "/checkout/compiler/rustc_mir_transform/src/single_enum.rs" "/checkout/compiler/rustc_mir_transform/src/check_const_item_mutation.rs" "/checkout/compiler/rustc_mir_transform/src/const_goto.rs" "/checkout/compiler/rustc_mir_transform/src/lower_intrinsics.rs" "/checkout/compiler/rustc_mir_transform/src/match_branches.rs" "/checkout/compiler/rustc_mir_transform/src/const_debuginfo.rs" "/checkout/compiler/rustc_mir_transform/src/remove_uninit_drops.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
