plain
   Compiling rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
   Compiling rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
   Compiling rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
   Compiling rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error[E0603]: unit struct `DefaultMetadataLoader` is private
    |
    |
253 |         Box::new(rustc_codegen_ssa::back::metadata::DefaultMetadataLoader)
    |                                                     ^^^^^^^^^^^^^^^^^^^^^ private unit struct
    |
note: the unit struct `DefaultMetadataLoader` is defined here
error: unused import: `crate::llvm`
 --> compiler/rustc_codegen_llvm/src/metadata.rs:1:5
  |
1 | use crate::llvm;
1 | use crate::llvm;
  |     ^^^^^^^^^^^
  |
  = note: `-D unused-imports` implied by `-D warnings`

error: unused import: `crate::llvm::archive_ro::ArchiveRO`
 --> compiler/rustc_codegen_llvm/src/metadata.rs:2:5
  |
2 | use crate::llvm::archive_ro::ArchiveRO;


error: unused imports: `False`, `ObjectFile`, `mk_section_iter`
 --> compiler/rustc_codegen_llvm/src/metadata.rs:3:19
  |
3 | use crate::llvm::{mk_section_iter, False, ObjectFile};


error: unused import: `rustc_middle::middle::cstore::MetadataLoader`
 --> compiler/rustc_codegen_llvm/src/metadata.rs:4:5
4 | use rustc_middle::middle::cstore::MetadataLoader;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error: unused import: `rustc_codegen_ssa::METADATA_FILENAME`
 --> compiler/rustc_codegen_llvm/src/metadata.rs:7:5
7 | use rustc_codegen_ssa::METADATA_FILENAME;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: unused import: `rustc_data_structures::owning_ref::OwningRef`
error: unused import: `rustc_data_structures::owning_ref::OwningRef`
 --> compiler/rustc_codegen_llvm/src/metadata.rs:8:5
  |
8 | use rustc_data_structures::owning_ref::OwningRef;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: unused import: `rustc_data_structures::rustc_erase_owner`
 --> compiler/rustc_codegen_llvm/src/metadata.rs:9:5
9 | use rustc_data_structures::rustc_erase_owner;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: unused import: `tracing::debug`
---

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "jemalloc llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build --target=x86_64-unknown-linux-gnu --host=x86_64-unknown-linux-gnu --stage 2 library/std --rust-profile-generate=/tmp/rustc-pgo
Build completed unsuccessfully in 0:10:57
