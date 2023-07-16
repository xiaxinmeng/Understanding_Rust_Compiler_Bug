plain
   Compiling lsp-server v0.6.0 (/checkout/src/tools/rust-analyzer/lib/lsp-server)
   Compiling flycheck v0.0.0 (/checkout/src/tools/rust-analyzer/crates/flycheck)
   Compiling base-db v0.0.0 (/checkout/src/tools/rust-analyzer/crates/base-db)
   Compiling flate2 v1.0.24
   Compiling sourcegen v0.0.0 (/checkout/src/tools/rust-analyzer/crates/sourcegen)
   Compiling project-model v0.0.0 (/checkout/src/tools/rust-analyzer/crates/project-model)
   Compiling hir-def v0.0.0 (/checkout/src/tools/rust-analyzer/crates/hir-def)
   Compiling cfg v0.0.0 (/checkout/src/tools/rust-analyzer/crates/cfg)
   Compiling hir-ty v0.0.0 (/checkout/src/tools/rust-analyzer/crates/hir-ty)
---
   Compiling ide-db v0.0.0 (/checkout/src/tools/rust-analyzer/crates/ide-db)
   Compiling ide-diagnostics v0.0.0 (/checkout/src/tools/rust-analyzer/crates/ide-diagnostics)
   Compiling ide-assists v0.0.0 (/checkout/src/tools/rust-analyzer/crates/ide-assists)
   Compiling parser v0.0.0 (/checkout/src/tools/rust-analyzer/crates/parser)
   Compiling xtask v0.1.0 (/checkout/src/tools/rust-analyzer/xtask)
   Compiling cargo-platform v0.1.2
   Compiling cargo_metadata v0.15.0
   Compiling proc-macro-test v0.0.0 (/checkout/src/tools/rust-analyzer/crates/proc-macro-test)
error: failed to run custom build command for `proc-macro-test v0.0.0 (/checkout/src/tools/rust-analyzer/crates/proc-macro-test)`
error: failed to run custom build command for `proc-macro-test v0.0.0 (/checkout/src/tools/rust-analyzer/crates/proc-macro-test)`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/build/proc-macro-test-09e634e8af344c24/build-script-build` (exit status: 101)
  --- stderr
  thread 'main' panicked at 'assertion failed: output.status.success()', crates/proc-macro-test/build.rs:32:5
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:24:57
