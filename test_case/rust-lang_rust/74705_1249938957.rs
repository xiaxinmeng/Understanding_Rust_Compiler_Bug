
 Cargo.lock                                                      |   1 -
 compiler/rustc_codegen_llvm/Cargo.toml                          |   1 -
 compiler/rustc_codegen_llvm/src/back/lto.rs                     |  63 ++-----------
 compiler/rustc_codegen_llvm/src/back/write.rs                   | 258 +++--------------------------------------------------
 compiler/rustc_codegen_llvm/src/lib.rs                          |   2 +-
 compiler/rustc_codegen_llvm/src/llvm/ffi.rs                     |  74 +--------------
 compiler/rustc_codegen_llvm/src/llvm_util.rs                    |  29 +-----
 compiler/rustc_codegen_ssa/src/back/write.rs                    |   2 -
 compiler/rustc_interface/src/tests.rs                           |   1 -
 compiler/rustc_llvm/llvm-wrapper/PassWrapper.cpp                | 235 +-----------------------------------------------
 compiler/rustc_session/src/options.rs                           |   2 -
 src/doc/unstable-book/src/compiler-flags/self-profile-events.md |   2 +-
 src/test/rustdoc-ui/z-help.stdout                               |   1 -
 src/test/ui/invalid/invalid-llvm-passes.rs                      |   2 +-
 src/test/ui/sanitize/new-llvm-pass-manager-thin-lto.rs          |   2 +-
 15 files changed, 24 insertions(+), 651 deletions(-)
