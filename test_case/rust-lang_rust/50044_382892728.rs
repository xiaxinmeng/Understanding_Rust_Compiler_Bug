plain
[00:22:10]    Compiling rustc_trans v0.0.0 (file:///checkout/src/librustc_trans)
[00:22:10]    Compiling cc v1.0.9
[00:22:10]    Compiling num_cpus v1.8.0
[00:22:13]    Compiling rustc_llvm v0.0.0 (file:///checkout/src/librustc_llvm)
[00:22:24] error: could not find native static library `Polly`, perhaps an -L flag is missing?
[00:22:24] error: aborting due to previous error
[00:22:24] 
[00:22:25] error: Could not compile `rustc_llvm`.
[00:22:25] 
[00:22:25] 
[00:22:25] Caused by:
[00:22:25]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_llvm librustc_llvm/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=4b8cacd6b6b8cbfa -C extra-filename=-4b8cacd6b6b8cbfa --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblibc-9b9c9af3e08f5b49.rlib --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-9866e194db82a141.rlib --extern rustc_cratesio_shim=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_cratesio_shim-296054b03e3b45fe.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/rustc_llvm-72b7d72c0f85e5c7/out -L native=/usr/lib/llvm-3.9/lib --cfg llvm_component="aarch64" --cfg llvm_component="arm" --cfg llvm_component="asmparser" --cfg llvm_component="bitreader" --cfg llvm_component="bitwriter" --cfg llvm_component="instrumentation" --cfg llvm_component="interpreter" --cfg llvm_component="ipo" --cfg llvm_component="linker" --cfg llvm_component="lto" --cfg llvm_component="mcjit" --cfg llvm_component="mips" --cfg llvm_component="msp430" --cfg llvm_component="nvptx" --cfg llvm_component="powerpc" --cfg llvm_component="sparc" --cfg llvm_component="systemz" --cfg llvm_component="x86" -l static=rustllvm -l dylib=LLVM-3.9 -l dylib=rt -l dylib=dl -l dylib=tinfo -l dylib=pthread -l dylib=z -l dylib=m -l static=Polly -l static=PollyISL -l stdc++` (exit code: 101)
[00:22:25] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/librustc_trans/Cargo.toml" "--features" " jemalloc" "--message-format" "json"
[00:22:25] expected success, got: exit code: 101
[00:22:25] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1094:9
[00:22:25] travis_fold:start:stage0-rustc_trans
travis_time:start:stage0-rustc_trans
travis_fold:end:stage0-rustc_trans


[00:22:25] travis_time:end:stage0-rustc_trans:start=1524174858966795452,finish=1524174873538139900,duration=14571344448

[00:22:25] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:22:25] Build completed unsuccessfully in 0:17:32
[00:22:25] Makefile:28: recipe for target 'all' failed
[00:22:25] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:09ecb54b
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
