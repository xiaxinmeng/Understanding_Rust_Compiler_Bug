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
Diff in /checkout/compiler/rustc_trait_selection/src/traits/specialize/mod.rs at line 349:
         E0751,
         "found both positive and negative implementation of trait `{}`{}:",
         overlap.trait_desc,
-            .self_desc
-            .clone()
-            .clone()
-            .map_or_else(|| String::new(), |ty| format!(" for type `{}`", ty))
+        overlap.self_desc.clone().map_or_else(|| String::new(), |ty| format!(" for type `{}`", ty))
     );
 
     match tcx.span_of_impl(negative_impl_def_id) {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_trait_selection/src/traits/specialize/mod.rs" "/checkout/compiler/rustc_trait_selection/src/traits/specialize/specialization_graph.rs" "/checkout/compiler/rustc_trait_selection/src/traits/coherence.rs" "/checkout/compiler/rustc_trait_selection/src/traits/on_unimplemented.rs" "/checkout/compiler/rustc_trait_selection/src/traits/structural_match.rs" "/checkout/compiler/rustc_trait_selection/src/traits/auto_trait.rs" "/checkout/compiler/rustc_trait_selection/src/traits/chalk_fulfill.rs" "/checkout/compiler/rustc_target/src/abi/call/nvptx64.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:15
