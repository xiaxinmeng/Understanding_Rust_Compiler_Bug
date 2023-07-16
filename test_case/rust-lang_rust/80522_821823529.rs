plain
Set({"compiler/rustc_macros"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Set({"compiler/rustc_metadata"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Set({"compiler/rustc_middle"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Set({"compiler/rustc_mir_build"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Set({"compiler/rustc_mir_dataflow"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Set({"compiler/rustc_mir_transform"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Set({"compiler/rustc_mir_transform/src/coverage/test_macros"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Set({"compiler/rustc_monomorphize"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Set({"compiler/rustc_parse_format"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Set({"compiler/rustc_passes"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Set({"compiler/rustc_plugin_impl"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Set({"compiler/rustc_privacy"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
---
Set({"compiler/rustc_macros"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"compiler/rustc_metadata"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"compiler/rustc_middle"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"compiler/rustc_mir_build"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"compiler/rustc_mir_dataflow"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"compiler/rustc_mir_transform"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"compiler/rustc_mir_transform/src/coverage/test_macros"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"compiler/rustc_monomorphize"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"compiler/rustc_parse_format"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"compiler/rustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"compiler/rustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"compiler/rustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
---
   Compiling rustc_mir_dataflow v0.0.0 (/checkout/compiler/rustc_mir_dataflow)
   Compiling rustc_metadata v0.0.0 (/checkout/compiler/rustc_metadata)
   Compiling rustc_incremental v0.0.0 (/checkout/compiler/rustc_incremental)
   Compiling rustc_symbol_mangling v0.0.0 (/checkout/compiler/rustc_symbol_mangling)
   Compiling rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
   Compiling rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
   Compiling rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
   Compiling rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
   Compiling rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
---
   Compiling rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
   Compiling rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
   Compiling rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
   Compiling rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
   Compiling rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
   Compiling rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
   Compiling rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
   Compiling rustc_interface v0.0.0 (/checkout/compiler/rustc_interface)
   Compiling rustc_driver v0.0.0 (/checkout/compiler/rustc_driver)
---
   Compiling rustc_mir_dataflow v0.0.0 (/checkout/compiler/rustc_mir_dataflow)
   Compiling rustc_metadata v0.0.0 (/checkout/compiler/rustc_metadata)
   Compiling rustc_incremental v0.0.0 (/checkout/compiler/rustc_incremental)
   Compiling rustc_symbol_mangling v0.0.0 (/checkout/compiler/rustc_symbol_mangling)
   Compiling rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
   Compiling rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error: the feature `or_patterns` has been stable since 1.53.0 and no longer requires an attribute to enable
 --> compiler/rustc_monomorphize/src/lib.rs:5:12
  |
  |
5 | #![feature(or_patterns)]
  |            ^^^^^^^^^^^
  |
  = note: `-D stable-features` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `rustc_monomorphize`

---
  |
9 | #![feature(or_patterns)]
  |            ^^^^^^^^^^^
  |
  = note: `-D stable-features` implied by `-D warnings`
error: aborting due to previous error

error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
