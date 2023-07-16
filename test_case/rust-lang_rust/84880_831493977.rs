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
Diff in /checkout/src/librustdoc/clean/utils.rs at line 450:
             (cx.tcx.parent(i).expect("cannot get parent def id"), ItemType::Enum)
         }
         // Each of these have their own page.
-        Res::Def(kind @ (Fn | TyAlias | Enum | Trait | Struct | Union | Mod | ForeignTy | Const | Static | Macro(..) | TraitAlias), i) => (i, kind.into()),
+        Res::Def(
+            @
+            @
+            (Fn | TyAlias | Enum | Trait | Struct | Union | Mod | ForeignTy | Const | Static
+            | Macro(..) | TraitAlias),
+            i,
+        ) => (i, kind.into()),
         // This is part of a trait definition; document the trait.
         Res::SelfTy(Some(trait_def_id), _) => (trait_def_id, ItemType::Trait),
         // This is an inherent impl; it doesn't have its own page.
Diff in /checkout/src/librustdoc/clean/utils.rs at line 457:
         Res::SelfTy(None, Some((impl_def_id, _))) => return impl_def_id,
-        Res::SelfTy(None, None) | Res::PrimTy(_) | Res::ToolMod | Res::SelfCtor(_) | Res::Local(_) | Res::NonMacroAttr(_) | Res::Err => return res.def_id(),
-        Res::Def(TyParam | ConstParam | Ctor(..) | ExternCrate | Use | ForeignMod | AnonConst | OpaqueTy | Field | LifetimeParam | GlobalAsm | Impl | Closure | Generator, id) => return id,
+        Res::SelfTy(None, None)
+        | Res::PrimTy(_)
+        | Res::ToolMod
+        | Res::SelfCtor(_)
+        | Res::Local(_)
+        | Res::NonMacroAttr(_)
+        | Res::Err => return res.def_id(),
+        Res::Def(
+            TyParam | ConstParam | Ctor(..) | ExternCrate | Use | ForeignMod | AnonConst | OpaqueTy
+            | Field | LifetimeParam | GlobalAsm | Impl | Closure | Generator,
+            id,
+        ) => return id,
     };
     if did.is_local() {
         return did;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/clean/blanket_impl.rs" "/checkout/src/librustdoc/clean/mod.rs" "/checkout/src/librustdoc/visit_lib.rs" "/checkout/src/librustdoc/doctree.rs" "/checkout/src/etc/test-float-parse/rand-f64.rs" "/checkout/src/etc/test-float-parse/huge-pow10.rs" "/checkout/src/etc/test-float-parse/many-digits.rs" "/checkout/src/librustdoc/clean/utils.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
