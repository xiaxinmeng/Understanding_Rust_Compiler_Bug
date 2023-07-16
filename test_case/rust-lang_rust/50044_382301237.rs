plain
[00:00:47] configure: rust.quiet-tests     := True
---
[00:18:30] error: could not find native static library `Polly`, perhaps an -L flag is missing?
---
[00:18:30]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_llvm librustc_llvm/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=4b8cacd6b6b8cbfa -C extra-filename=-4b8cacd6b6b8cbfa --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern rustc_cratesio_shim=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_cratesio_shim-296054b03e3b45fe.so --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-9866e194db82a141.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblibc-9b9c9af3e08f5b49.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/rustc_llvm-72b7d72c0f85e5c7/out -L native=/usr/lib/llvm-3.9/lib --cfg llvm_component="aarch64" --cfg llvm_component="arm" --cfg llvm_component="asmparser" --cfg llvm_component="bitreader" --cfg llvm_component="bitwriter" --cfg llvm_component="instrumentation" --cfg llvm_component="interpreter" --cfg llvm_component="ipo" --cfg llvm_component="linker" --cfg llvm_component="lto" --cfg llvm_component="mcjit" --cfg llvm_component="mips" --cfg llvm_component="msp430" --cfg llvm_component="nvptx" --cfg llvm_component="powerpc" --cfg llvm_component="sparc" --cfg llvm_component="systemz" --cfg llvm_component="x86" -l static=rustllvm -l dylib=LLVM-3.9 -l dylib=rt -l dylib=dl -l dylib=tinfo -l dylib=pthread -l dylib=z -l dylib=m -l static=Polly -l static=PollyISL -l stdc++` (exit code: 101)
[00:18:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/librustc_trans/Cargo.toml" "--features" " jemalloc" "--message-format" "json"
[00:18:30] expected success, got: exit code: 101
[00:18:30] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1088:9
---
[00:18:30] make: *** [all] Error 1
[00:18:30] Makefile:28: recipe for targ2387044 .
---
122348 ./obj/build/bootstrap/debug/incremental/bootstrap-351vorei3hhuv/s-f085adqnq4-2j9f6r-3cdwuqmwdwxyb
---
24784 ./src/libcompicannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:109acb82:start=1524038771284420258,finish=1524038771289953712,duration=5533454
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:01377c68
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:01377c68:start=1524038771294847837,finish=1524038771300640063,duration=5792226
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:26ca8967
$ dmesg | grep -i kill
[   10.013125] init: failsafe main process (1093) killed by TERM signal
[   41.231828] init: plymouth-upstart-bridge main process (509) killed by TERM signal
