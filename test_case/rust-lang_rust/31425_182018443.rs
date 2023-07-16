
error: error pattern ' build without -C codegen-units for more exact errors' not found!
status: exit code: 101
command: x86_64-unknown-linux-gnu/stage2/bin/rustc /buildslave/rust-buildbot/slave/auto-linux-64-opt/build/src/test/compile-fail/asm-src-loc-codegen-units.rs -L x86_64-unknown-linux-gnu/test/compile-fail/ --target=x86_64-unknown-linux-gnu -L x86_64-unknown-linux-gnu/test/compile-fail/asm-src-loc-codegen-units.stage2-x86_64-unknown-linux-gnu.compile-fail.libaux -C prefer-dynamic -o x86_64-unknown-linux-gnu/test/compile-fail/asm-src-loc-codegen-units.stage2-x86_64-unknown-linux-gnu --cfg rtopt -C rpath -O -L x86_64-unknown-linux-gnu/rt -C codegen-units=2
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
thread 'error: codegen-1<inline asm>:1:2: error: invalid instruction mnemonic 'nowayisthisavalidinstruction'
        nowayisthisavalidinstruction
        ^
' panicked at 'Box<Any>', 
../src/libsyntax/errors/mod.rs:note607: 
build without -C codegen-units for more exact errorsnote: Run with `RUST_BACKTRACE=1` for a backtrace.

error: aborting due to previous error
error: aborting due to worker thread panic

------------------------------------------

thread '[compile-fail] compile-fail/asm-src-loc-codegen-units.rs' panicked at 'explicit panic', /buildslave/rust-buildbot/slave/auto-linux-64-opt/build/src/compiletest/runtest.rs:1527
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    [compile-fail] compile-fail/asm-src-loc-codegen-units.rs
