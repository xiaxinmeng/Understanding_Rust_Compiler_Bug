
thread 'main' panicked at '
command did not execute successfully, got: exit status: 1

build script failed, must exit now', /home/daira/.cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.44/src/lib.rs:885:5
stack backtrace:
   0: rust_begin_unwind
             at /rustc/e784c962ea252f0874a4305168077e7048cb39e9/library/std/src/panicking.rs:517:5
   1: std::panicking::begin_panic_fmt
             at /rustc/e784c962ea252f0874a4305168077e7048cb39e9/library/std/src/panicking.rs:460:5
   2: cmake::fail
             at /home/daira/.cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.44/src/lib.rs:885:5
   3: cmake::run
             at /home/daira/.cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.44/src/lib.rs:863:9
   4: cmake::Config::build
             at /home/daira/.cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.44/src/lib.rs:698:13
   5: <bootstrap::native::Llvm as bootstrap::builder::Step>::run
             at ./src/bootstrap/native.rs:359:9
   6: bootstrap::builder::Builder::ensure
             at ./src/bootstrap/builder.rs:1580:23
   7: bootstrap::compile::rustc_cargo_env
             at ./src/bootstrap/compile.rs:700:27
   8: bootstrap::compile::rustc_cargo
             at ./src/bootstrap/compile.rs:652:5
   9: <bootstrap::compile::Rustc as bootstrap::builder::Step>::run
             at ./src/bootstrap/compile.rs:588:9
  10: bootstrap::builder::Builder::ensure
             at ./src/bootstrap/builder.rs:1580:23
  11: <bootstrap::compile::Assemble as bootstrap::builder::Step>::run
             at ./src/bootstrap/compile.rs:1075:9
  12: bootstrap::builder::Builder::ensure
             at ./src/bootstrap/builder.rs:1580:23
  13: bootstrap::builder::Builder::compiler
             at ./src/bootstrap/builder.rs:627:9
  14: <bootstrap::compile::Std as bootstrap::builder::Step>::make_run
             at ./src/bootstrap/compile.rs:52:23
  15: bootstrap::builder::StepDescription::maybe_run
             at ./src/bootstrap/builder.rs:175:13
  16: bootstrap::builder::StepDescription::run
             at ./src/bootstrap/builder.rs:211:25
  17: bootstrap::builder::Builder::run_step_descriptions
             at ./src/bootstrap/builder.rs:619:9
  18: bootstrap::builder::Builder::execute_cli
             at ./src/bootstrap/builder.rs:599:9
  19: bootstrap::Build::build
             at ./src/bootstrap/lib.rs:624:13
  20: bootstrap::main
             at ./src/bootstrap/bin/main.rs:33:5
  21: core::ops::function::FnOnce::call_once
             at /rustc/e784c962ea252f0874a4305168077e7048cb39e9/library/core/src/ops/function.rs:227:5
