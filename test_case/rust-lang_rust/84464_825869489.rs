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
Diff in /checkout/src/librustdoc/clean/utils.rs at line 1:
 use crate::clean::auto_trait::AutoTraitFinder;
 use crate::clean::blanket_impl::BlanketImplFinder;
 use crate::clean::{
-    inline, Clean, Crate, Generic, GenericArg, GenericArgs, ImportSource, Item,
-    ItemKind, Lifetime, MacroKind, Path, PathSegment, Primitive, PrimitiveType, ResolvedPath, Type,
-    TypeBinding,
+    inline, Clean, Crate, Generic, GenericArg, GenericArgs, ImportSource, Item, ItemKind, Lifetime,
+    MacroKind, Path, PathSegment, Primitive, PrimitiveType, ResolvedPath, Type, TypeBinding,
 use crate::core::DocContext;
 use crate::formats::item_type::ItemType;
 use crate::formats::item_type::ItemType;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/etc/test-float-parse/short-decimals.rs" "/checkout/src/librustdoc/clean/inline.rs" "/checkout/src/librustdoc/clean/simplify.rs" "/checkout/src/librustdoc/clean/cfg/tests.rs" "/checkout/src/librustdoc/clean/cfg.rs" "/checkout/src/librustdoc/clean/utils.rs" "/checkout/src/librustdoc/clean/blanket_impl.rs" "/checkout/src/etc/test-float-parse/many-digits.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:15
