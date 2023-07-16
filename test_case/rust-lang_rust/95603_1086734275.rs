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
Diff in /checkout/compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs at line 1161:
         } else {
             // Substitute the type, so we can print a fixup given `type Alias = dyn Trait`
             let name = liberated_sig.output().to_string();
-            let name = name.strip_prefix('(').and_then(|name| name.strip_suffix(')')).unwrap_or(&name);
+            let name =
+                name.strip_prefix('(').and_then(|name| name.strip_suffix(')')).unwrap_or(&name);
             if !name.starts_with("dyn ") {
             }
             }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_trait_selection/src/traits/specialize/specialization_graph.rs" "/checkout/compiler/rustc_trait_selection/src/traits/specialize/mod.rs" "/checkout/compiler/rustc_trait_selection/src/traits/fulfill.rs" "/checkout/compiler/rustc_trait_selection/src/traits/project.rs" "/checkout/compiler/rustc_trait_selection/src/traits/chalk_fulfill.rs" "/checkout/compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs" "/checkout/compiler/rustc_trait_selection/src/traits/error_reporting/on_unimplemented.rs" "/checkout/compiler/rustc_trait_selection/src/traits/util.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
