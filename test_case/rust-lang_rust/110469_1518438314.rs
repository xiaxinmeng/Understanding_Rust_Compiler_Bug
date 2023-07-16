plain
    Finished release [optimized] target(s) in 24.38s
[TIMING] tool::ToolBuild { compiler: Compiler { stage: 0, host: aarch64-unknown-linux-gnu }, target: aarch64-unknown-linux-gnu, tool: "tidy", path: "src/tools/tidy", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: [], allow_features: "" } -- 24.421
[TIMING] tool::Tidy { compiler: Compiler { stage: 0, host: aarch64-unknown-linux-gnu }, target: aarch64-unknown-linux-gnu } -- 0.000
fmt check
Diff in /checkout/compiler/rustc_metadata/src/rmeta/encoder.rs at line 841:
         | DefKind::Impl { .. }
         | DefKind::Closure
         | DefKind::Generator => true,
-        DefKind::ForeignMod
-        | DefKind::ImplTraitPlaceholder
-        | DefKind::GlobalAsm => false,
+        DefKind::ForeignMod | DefKind::ImplTraitPlaceholder | DefKind::GlobalAsm => false,
 }
 
 
Running `"/checkout/obj/build/aarch64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_metadata/src/rmeta/encoder.rs" "/checkout/compiler/rustc_metadata/src/rmeta/decoder/cstore_impl.rs" "/checkout/compiler/rustc_metadata/src/rmeta/table.rs" "/checkout/compiler/rustc_metadata/src/rmeta/mod.rs" "/checkout/compiler/rustc_metadata/src/rmeta/decoder.rs" "/checkout/compiler/rustc_query_impl/src/profiling_support.rs" "/checkout/compiler/rustc_metadata/src/locator.rs" "/checkout/compiler/rustc_metadata/src/rmeta/def_path_hash_map.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
