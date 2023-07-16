plain
   Compiling semver v0.11.0
   Compiling cargo_metadata v0.12.0
[RUSTC-TIMING] semver test:false 0.804
   Compiling clippy_lints v0.1.52 (/checkout/src/tools/clippy/clippy_lints)
error: top-level or-patterns are not allowed in `let` bindings
    |
    |
888 |         let Sugg::NonParen(s) | Sugg::MaybeParen(s) | Sugg::BinOp(_, s) = &self.0;
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(Sugg::NonParen(s) | Sugg::MaybeParen(s) | Sugg::BinOp(_, s))`
[RUSTC-TIMING] semver_parser test:false 2.590
[RUSTC-TIMING] cargo_metadata test:false 6.616
error: aborting due to previous error

---
[RUSTC-TIMING] futures_macro test:false 1.580
   Compiling tokio v0.2.24
[RUSTC-TIMING] json test:false 3.894
   Compiling clippy_lints v0.1.52 (/checkout/src/tools/clippy/clippy_lints)
error: top-level or-patterns are not allowed in `let` bindings
    |
    |
888 |         let Sugg::NonParen(s) | Sugg::MaybeParen(s) | Sugg::BinOp(_, s) = &self.0;
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(Sugg::NonParen(s) | Sugg::MaybeParen(s) | Sugg::BinOp(_, s))`
[RUSTC-TIMING] futures_macro test:false 1.658
[RUSTC-TIMING] rustc_workspace_hack test:false 0.031
   Compiling futures-util v0.3.12
[RUSTC-TIMING] env_logger test:false 3.230
---
Dist rust-analyzer-nightly-mipsel-unknown-linux-gnu
 finished in 10.046 seconds
[TIMING] RustAnalyzer { compiler: Compiler { stage: 1, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } }, target: TargetSelection { triple: "mipsel-unknown-linux-gnu", file: None } } -- 10.078
Dist llvm-tools-nightly-mipsel-unknown-linux-gnu
thread 'main' panicked at 'clippy expected to build - essential tool', src/bootstrap/dist.rs:1129:14
 finished in 23.973 seconds
[TIMING] LlvmTools { target: TargetSelection { triple: "mipsel-unknown-linux-gnu", file: None } } -- 24.172
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host mipsel-unknown-linux-gnu --target mipsel-unknown-linux-gnu
Build completed unsuccessfully in 0:24:12
