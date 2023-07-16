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
Diff in /checkout/src/librustdoc/clean/mod.rs at line 120:
             hir::GenericBound::Trait(ref t, modifier) => {
                 // `T: ~const Drop` is not equivalent to `T: Drop`, and we don't currently document `~const` bounds
                 // because of its experimental status, so just don't show these.
-                if Some(t.trait_ref.trait_def_id().unwrap())
-                    == cx.tcx.lang_items().drop_trait()
+                if Some(t.trait_ref.trait_def_id().unwrap()) == cx.tcx.lang_items().drop_trait()
                     && hir::TraitBoundModifier::MaybeConst == modifier
                     return None;
Diff in /checkout/src/librustdoc/clean/mod.rs at line 2112:
Diff in /checkout/src/librustdoc/clean/mod.rs at line 2112:
             hir::TypeBindingKind::Equality { ref ty } => {
                 TypeBindingKind::Equality { ty: ty.clean(cx) }
             }
-            hir::TypeBindingKind::Constraint { bounds } => {
-                TypeBindingKind::Constraint { bounds: bounds.iter().filter_map(|b| b.clean(cx)).collect() }
-            }
+            hir::TypeBindingKind::Constraint { bounds } => TypeBindingKind::Constraint {
+                bounds: bounds.iter().filter_map(|b| b.clean(cx)).collect(),
         }
     }
 }
 }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/json/mod.rs" "/checkout/src/librustdoc/clean/utils/tests.rs" "/checkout/src/librustdoc/clean/cfg.rs" "/checkout/src/librustdoc/clean/mod.rs" "/checkout/src/librustdoc/clean/types.rs" "/checkout/src/librustdoc/visit.rs" "/checkout/src/librustdoc/scrape_examples.rs" "/checkout/src/librustdoc/clean/simplify.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
