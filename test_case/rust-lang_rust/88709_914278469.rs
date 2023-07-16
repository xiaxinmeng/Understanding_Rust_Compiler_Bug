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
Diff in /checkout/compiler/rustc_trait_selection/src/traits/const_evaluatable.rs at line 579:
                 && iter::zip(a_args, b_args)
                     .all(|(&an, &bn)| try_unify(tcx, a.subtree(an), b.subtree(bn)))
         }
-        (Node::Cast(a_kind, a_operand, a_ty), Node::Cast(b_kind, b_operand, b_ty)) if (a_ty == b_ty) && (a_kind == b_kind) => {
+        (Node::Cast(a_kind, a_operand, a_ty), Node::Cast(b_kind, b_operand, b_ty))
+            if (a_ty == b_ty) && (a_kind == b_kind) =>
+        {
             try_unify(tcx, a.subtree(a_operand), b.subtree(b_operand))
         }
         // use this over `_ => false` to make adding variants to `Node` less error prone
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_trait_selection/src/traits/coherence.rs" "/checkout/compiler/rustc_trait_selection/src/traits/chalk_fulfill.rs" "/checkout/compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs" "/checkout/compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs" "/checkout/compiler/rustc_trait_selection/src/traits/error_reporting/on_unimplemented.rs" "/checkout/compiler/rustc_trait_selection/src/traits/util.rs" "/checkout/compiler/rustc_trait_selection/src/traits/const_evaluatable.rs" "/checkout/compiler/rustc_trait_selection/src/lib.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
