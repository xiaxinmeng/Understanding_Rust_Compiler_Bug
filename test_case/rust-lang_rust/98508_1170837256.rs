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

######################################################################## 100.0%
extracting /checkout/obj/build/cache/2022-05-20/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz to /checkout/obj/build/x86_64-unknown-linux-gnu/stage0
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/compiler/rustc_trait_selection/src/traits/const_evaluatable.rs at line 672:
         let root = ct.root(tcx);
         if let Node::Leaf(leaf) = root {
             if let ty::ConstKind::Unevaluated(_) = leaf.kind() {
-              return ControlFlow::CONTINUE;
+                return ControlFlow::CONTINUE;
         };
         };
         f(ct)?;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_builtin_macros/src/edition_panic.rs" "/checkout/compiler/rustc_trait_selection/src/traits/relationships.rs" "/checkout/compiler/rustc_builtin_macros/src/proc_macro_harness.rs" "/checkout/compiler/rustc_trait_selection/src/traits/engine.rs" "/checkout/compiler/rustc_builtin_macros/src/concat_idents.rs" "/checkout/compiler/rustc_trait_selection/src/traits/codegen.rs" "/checkout/compiler/rustc_builtin_macros/src/standard_library_imports.rs" "/checkout/compiler/rustc_trait_selection/src/traits/const_evaluatable.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
