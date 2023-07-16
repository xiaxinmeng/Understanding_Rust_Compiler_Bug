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
Diff in /checkout/compiler/rustc_mir_transform/src/inline.rs at line 17:
 use std::iter;
 use std::ops::{Range, RangeFrom};
 
-use crate::{LocalDefId};
+use crate::LocalDefId;
 use rustc_data_structures::stable_map::FxHashMap;
 use rustc_data_structures::sync::Lrc;
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir_transform/src/check_const_item_mutation.rs" "/checkout/compiler/rustc_mir_transform/src/instcombine.rs" "/checkout/compiler/rustc_mir_transform/src/inline.rs" "/checkout/compiler/rustc_mir_transform/src/dest_prop.rs" "/checkout/compiler/rustc_mir_transform/src/inline/cycle.rs" "/checkout/compiler/rustc_mir_transform/src/generator.rs" "/checkout/compiler/rustc_mir_transform/src/remove_storage_markers.rs" "/checkout/compiler/rustc_mir_transform/src/remove_unneeded_drops.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
