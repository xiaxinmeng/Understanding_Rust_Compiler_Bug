
192-168-1-4:wasm-game-of-life anton$ wasm-pack init
  
  [1/8] 🔧  Checking crate configuration...
  [2/8] 🎯  Adding WASM target...
\ [3/8] 🌀  Compiling to WASM...
Compilation of your program failed. stderr:

   Compiling wasm-game-of-life v0.1.0 (file:///Users/anton/projects/hacks/rust/wasm-game-of-life)
warning: function is never used: `set_panic_hook`
  --> src/utils.rs:11:9
   |
11 |         pub fn set_panic_hook() {}
   |         ^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: #[warn(dead_code)] on by default

error: linking with `rust-lld` failed: signal: 6
  |
  = note: "rust-lld" "-flavor" "wasm" "-L" "/Users/anton/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/wasm32-unknown-unknown/lib" "/Users/anton/projects/hacks/rust/wasm-game-of-life/target/wasm32-unknown-unknown/release/deps/wasm_game_of_life.wasm_game_of_life.59gcvxac-cgu.0.rcgu.o" "/Users/anton/projects/hacks/rust/wasm-game-of-life/target/wasm32-unknown-unknown/release/deps/wasm_game_of_life.wasm_game_of_life.59gcvxac-cgu.1.rcgu.o" "/Users/anton/projects/hacks/rust/wasm-game-of-life/target/wasm32-unknown-unknown/release/deps/wasm_game_of_life.wasm_game_of_life.59gcvxac-cgu.2.rcgu.o" "-o" "/Users/anton/projects/hacks/rust/wasm-game-of-life/target/wasm32-unknown-unknown/release/deps/wasm_game_of_life.wasm" "/Users/anton/projects/hacks/rust/wasm-game-of-life/target/wasm32-unknown-unknown/release/deps/wasm_game_of_life.bxpq0uzydj2zf4e.rcgu.o" "--gc-sections" "-O3" "-L" "/Users/anton/projects/hacks/rust/wasm-game-of-life/target/wasm32-unknown-unknown/release/deps" "-L" "/Users/anton/projects/hacks/rust/wasm-game-of-life/target/release/deps" "-L" "/Users/anton/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/wasm32-unknown-unknown/lib" "/Users/anton/projects/hacks/rust/wasm-game-of-life/target/wasm32-unknown-unknown/release/deps/libwasm_bindgen-f9f62e4f9e995bb5.rlib" "/Users/anton/projects/hacks/rust/wasm-game-of-life/target/wasm32-unknown-unknown/release/deps/libcfg_if-954cfacc8a40931a.rlib" "/Users/anton/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/wasm32-unknown-unknown/lib/libstd-b0efc82f64d03095.rlib" "/Users/anton/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/wasm32-unknown-unknown/lib/libpanic_abort-4f93017645c0d252.rlib" "/Users/anton/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/wasm32-unknown-unknown/lib/libunwind-c43a3d8cf78f9d5a.rlib" "/Users/anton/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/wasm32-unknown-unknown/lib/liblibc-68373f2b6bc18151.rlib" "/Users/anton/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/wasm32-unknown-unknown/lib/liballoc_system-40a64b71ec5a8f18.rlib" "/Users/anton/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/wasm32-unknown-unknown/lib/libdlmalloc-f6caffaf44cc60a0.rlib" "/Users/anton/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/wasm32-unknown-unknown/lib/liballoc-863f2b2485046aef.rlib" "/Users/anton/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/wasm32-unknown-unknown/lib/libcore-21433d3b7663b55b.rlib" "/Users/anton/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/wasm32-unknown-unknown/lib/libcompiler_builtins-bf63ca2c3bc4de53.rlib" "--no-threads" "-z" "stack-size=1048576" "--stack-first" "--allow-undefined" "--no-entry" "--export-table"
  = note: dyld: Library not loaded: @rpath/libLLVM.dylib
            Referenced from: /Users/anton/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/bin/rust-lld
            Reason: image not found
          

error: aborting due to previous error

error: Could not compile `wasm-game-of-life`.

To learn more, run the command again with --verbose.
