plain
[00:06:03]    Compiling rustc_data_structures v0.0.0 (file:///checkout/src/librustc_data_structures)
[00:06:06]    Compiling arena v0.0.0 (file:///checkout/src/libarena)
[00:06:06]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:06:10]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
/libbitflags-99b4534960f92449.rlib --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-5d0a8a65bb9fe29f.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-5d0a8a65bb9fe29f.rlib --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-0404335fb4ae3dc1.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-5073f1296cd24b67.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-05d59ed98ebd8949.so --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-b76c070114255d98.rlib --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-58741ed9de9aae4f.so` (exit code: 101)
[00:06:18] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:06:18] expected success, got: exit code: 101
[00:06:18] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:06:18] travis_fol2292740 .
1404348 ./obj
1404316 ./obj/build
803072 ./obj/build/x86_64-unknown-linux-gnu
---
147048 ./.git/modules
147044 ./.git/modules/src
137732 ./obj/build/bootstrap/debug/incremental
123180 ./obj/build/bootstrap/debug/incremental/bootstrap-1r3bppl29tbrj
123176 ./obj/build/bootstrap/debug/incremental/bootstrap-1r3bppl29tbrj/s-f1lz7zhofl-pihry7-2rxy29zuufd4c
89796 ./src/llvm/test/CodeGen
81804 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
72256 ./.git/modules/src/tools
70500 ./obj/build/x86_64-unknown-linux-gnu/native
