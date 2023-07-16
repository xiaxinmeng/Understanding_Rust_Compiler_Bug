plain
 Documenting rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
 Documenting rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
 Documenting rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
 Documenting rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
error: unresolved link to `ExprKind::BinaryOp`
   --> compiler/rustc_mir_build/src/thir/mod.rs:184:23
    |
184 |     /// [`BinaryOp`]: Self::BinaryOp
    |                       ^^^^^^^^^^^^^^ the enum `ExprKind` has no variant or associated item named `BinaryOp`
    |
    = note: `-D broken-intra-doc-links` implied by `-D warnings`
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
 Documenting rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error: aborting due to previous error


error: could not document `rustc_mir_build`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name rustc_mir_build compiler/rustc_mir_build/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-72d7e3c6a96c6a2b.rmeta --extern rustc_arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_arena-47b2920b8f65d36d.rmeta --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-6d310a5deca1ba1b.rmeta --extern rustc_attr=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_attr-0b9c2c96f9bcb552.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-db6f892351892c08.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-32d38bd2f1d0dc76.rmeta --extern rustc_hir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir-7c4762cc4983f658.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-53c75d98d366c3dd.rmeta --extern rustc_infer=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_infer-4d9c2843f674e51d.rmeta --extern rustc_middle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_middle-67499ad9502cfee5.rmeta --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_serialize-b9a24f71da7b43c7.rmeta --extern rustc_session=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-96df45e6985eefda.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-f4341b2568d12e83.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-363ddad2891513fc.rmeta --extern rustc_trait_selection=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_trait_selection-392b5d6ac62ae206.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-9fb44e28278df6b0.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libtracing-c382490ed46d3134.rmeta -Dwarnings -Winvalid_codeblock_attributes --crate-version '1.52.0-nightly
  (c344bcece
  2021-02-24)' --document-private-items --enable-index-page -Zunstable-options -Znormalize-docs` (exit code: 1)
warning: could not parse code block as Rust code
   --> compiler/rustc_typeck/src/check/upvar.rs:388:9
    |
388 |       /// 