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
Diff in /checkout/src/librustdoc/json/conversions.rs at line 8:
 use std::fmt;
 use rustc_ast::ast;
 use rustc_ast::ast;
-use rustc_hir::{def_id::DefId, def::CtorKind};
+use rustc_hir::{def::CtorKind, def_id::DefId};
 use rustc_middle::ty::TyCtxt;
 use rustc_span::def_id::CRATE_DEF_INDEX;
 use rustc_span::Pos;
Diff in /checkout/src/librustdoc/json/conversions.rs at line 182:
     match did {
     match did {
         ItemId::DefId(did) => Id(format!("{}", DisplayDefId(did))),
-        ItemId::Blanket { for_, trait_ } => Id(format!("b:{}-{}", DisplayDefId(trait_), DisplayDefId(for_))),
-        ItemId::Auto { for_, trait_ } => Id(format!("a:{}-{}", DisplayDefId(trait_), DisplayDefId(for_))),
+        ItemId::Blanket { for_, trait_ } => {
+            Id(format!("b:{}-{}", DisplayDefId(trait_), DisplayDefId(for_)))
+        }
+        ItemId::Auto { for_, trait_ } => {
+            Id(format!("a:{}-{}", DisplayDefId(trait_), DisplayDefId(for_)))
+        }
         ItemId::Primitive(krate) => Id(format!("p:{}", krate.as_u32())),
 }
 }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/alloc/benches/lib.rs" "/checkout/library/alloc/benches/vec.rs" "/checkout/library/alloc/benches/binary_heap.rs" "/checkout/library/alloc/benches/linked_list.rs" "/checkout/library/alloc/benches/str.rs" "/checkout/src/librustdoc/json/conversions.rs" "/checkout/src/librustdoc/docfs.rs" "/checkout/src/librustdoc/json/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
