
---- [compile-fail] compile-fail/asm-src-loc-codegen-units.rs stdout ----

error: error pattern ' build without -C codegen-units for more exact errors' not found!
status: exit code: 101
command: i686-unknown-linux-gnu/stage2/bin/rustc /home/ubuntu/src/rust-buildbot/slave/auto-linux-32-nopt-t/build/src/test/compile-fail/asm-src-loc-codegen-units.rs -L i686-unknown-linux-gnu/test/compile-fail/ --target=i686-unknown-linux-gnu -L i686-unknown-linux-gnu/test/compile-fail/asm-src-loc-codegen-units.stage2-i686-unknown-linux-gnulibaux -C prefer-dynamic -o i686-unknown-linux-gnu/test/compile-fail/asm-src-loc-codegen-units.stage2-i686-unknown-linux-gnu --cfg rtopt --cfg ndebug -L i686-unknown-linux-gnu/rt -C codegen-units=2
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
thread 'error: <inline asm>:1:2: error: invalid instruction mnemonic 'nowayisthisavalidinstruction'
        nowayisthisavalidinstruction
        ^
codegen-1
' panicked at 'Box<Any>note: ', /home/ubuntu/src/rust-buildbot/slave/auto-linux-32-nopt-t/build/src/libsyntax/diagnostic.rsbuild without -C codegen-units for more exact errors:
171
error: aborting due to previous error
error: aborting due to worker thread panic

------------------------------------------

thread '[compile-fail] compile-fail/asm-src-loc-codegen-units.rs' panicked at 'explicit panic', /home/ubuntu/src/rust-buildbot/slave/auto-linux-32-nopt-t/build/src/compiletest/runtest.rs:1499


