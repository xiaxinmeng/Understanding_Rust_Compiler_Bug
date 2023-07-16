plain
[00:26:40]    Compiling rustc_trans v0.0.0 (file:///checkout/src/librustc_trans)
[00:26:40]    Compiling cc v1.0.9
[00:26:40]    Compiling num_cpus v1.8.0
[00:26:44]    Compiling rustc_llvm v0.0.0 (file:///checkout/src/librustc_llvm)
[00:26:56] error: could not find native static library `Polly`, perhaps an -L flag is missing?
[00:26:56] error: aborting due to previous error
[00:26:56] 
[00:26:56] error: Could not compile `rustc_llvm`.
[00:26:56] 
[00:26:56] 
[00:26:56] Caused by:
[00:26:56]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_llvm librustc_llvm/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=4b8cacd6b6b8cbfa -C extra-filename=-4b8cacd6b6b8cbfa --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern rustc_cratesio_shim=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_cratesio_shim-296054b03e3b45fe.so --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblibc-9b9c9af3e08f5b49.rlib --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-9866e194db82a141.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/rustc_llvm-72b7d72c0f85e5c7/out -L native=/usr/lib/llvm-3.9/lib --cfg llvm_component="aarch64" --cfg llvm_component="arm"et 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00c95cc0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
