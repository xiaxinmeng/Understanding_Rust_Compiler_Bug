
error: linking with `rust-lld` failed: exit code: 1
  |
  = note: "rust-lld" "-flavor" "wasm" "--no-threads" "-z" "stack-size=1048576" "--stack-first" "--allow-undefined" "--fatal-warnings" "--no-demangle" "--export-dynamic" "--no-entry" "-L" "/home/omer/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/wasm32-unknown-unknown/lib" "-L" "/home/omer/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/wasm32-unknown-unknown/lib/self-contained" "foo.main.7rcbfp3g-cgu.0.rcgu.o" "foo.main.7rcbfp3g-cgu.1.rcgu.o" "-o" "foo.wasm" "--export" "start" "--export=__heap_base" "--export=__data_end" "--gc-sections" "-O0" "-L"
"/home/omer/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/wasm32-unknown-unknown/lib" "/home/omer/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/wasm32-unknown-unknown/lib/librustc_std_workspace_core-338ea5f3d59665eb.rlib" "/home/omer/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/wasm32-unknown-unknown/lib/libcore-c883bdbeb473297b.rlib" "/home/omer/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/wasm32-unknown-unknown/lib/libcompiler_builtins-4a8deb6b3def81fd.rlib" "--shared" "--import-memory"
  = note: rust-lld: error: foo.main.7rcbfp3g-cgu.1.rcgu.o: relocation R_WASM_MEMORY_ADDR_SLEB cannot be used against symbol .L__unnamed_1; recompile with -fPIC


error: aborting due to previous error
