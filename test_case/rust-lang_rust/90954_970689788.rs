plain
   Compiling proc-macro-error v1.0.4
error: could not compile `build_helper`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc --crate-name build_helper --edition=2021 src/build_helper/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=0 -C metadata=dded4ef7a66d7fd2 -C extra-filename=-dded4ef7a66d7fd2 --out-dir /checkout/obj/build/bootstrap/debug/deps -C incremental=/checkout/obj/build/bootstrap/debug/incremental -L dependency=/checkout/obj/build/bootstrap/debug/deps -Wrust_2018_idioms -Wunused_lifetimes -Wsemicolon_in_expressions_from_macros -Dwarnings` (signal: 11, SIGSEGV: invalid memory reference)
error: build failed
failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
Build completed unsuccessfully in 0:01:03
make: *** [prepare] Error 1
---
   Compiling toml v0.5.7
error: could not compile `bootstrap`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/stage0/bin/rustc --crate-name sccache_plus_cl --edition=2021 src/bootstrap/bin/sccache-plus-cl.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=1 -C metadata=b0fd04192a514335 -C extra-filename=-b0fd04192a514335 --out-dir /checkout/obj/build/tmp/distcheck/build/bootstrap/debug/deps -C incremental=/checkout/obj/build/tmp/distcheck/build/bootstrap/debug/incremental -L dependency=/checkout/obj/build/tmp/distcheck/build/bootstrap/debug/deps --extern bootstrap=/checkout/obj/build/tmp/distcheck/build/bootstrap/debug/deps/libbootstrap-b49b759706b6c45d.rlib --extern build_helper=/checkout/obj/build/tmp/distcheck/build/bootstrap/debug/deps/libbuild_helper-dded4ef7a66d7fd2.rlib --extern cc=/checkout/obj/build/tmp/distcheck/build/bootstrap/debug/deps/libcc-9022b5d7a536baba.rlib --extern cmake=/checkout/obj/build/tmp/distcheck/build/bootstrap/debug/deps/libcmake-a1cd7916c7902e9e.rlib --extern filetime=/checkout/obj/build/tmp/distcheck/build/bootstrap/debug/deps/libfiletime-21f7d6d27914ba34.rlib --extern getopts=/checkout/obj/build/tmp/distcheck/build/bootstrap/debug/deps/libgetopts-3bc5fa5912184ba0.rlib --extern ignore=/checkout/obj/build/tmp/distcheck/build/bootstrap/debug/deps/libignore-6eab7523daa887b2.rlib --extern lazy_static=/checkout/obj/build/tmp/distcheck/build/bootstrap/debug/deps/liblazy_static-1f5d3923e02f6398.rlib --extern libc=/checkout/obj/build/tmp/distcheck/build/bootstrap/debug/deps/liblibc-214e4f81fa8cd553.rlib --extern merge=/checkout/obj/build/tmp/distcheck/build/bootstrap/debug/deps/libmerge-ba43cbd72628adcc.rlib --extern num_cpus=/checkout/obj/build/tmp/distcheck/build/bootstrap/debug/deps/libnum_cpus-18e3b0eb22fae47b.rlib --extern once_cell=/checkout/obj/build/tmp/distcheck/build/bootstrap/debug/deps/libonce_cell-5bf50caf48c031d7.rlib --extern opener=/checkout/obj/build/tmp/distcheck/build/bootstrap/debug/deps/libopener-204d18544f7127ea.rlib --extern serde=/checkout/obj/build/tmp/distcheck/build/bootstrap/debug/deps/libserde-e7ed636813468773.rlib --extern serde_json=/checkout/obj/build/tmp/distcheck/build/bootstrap/debug/deps/libserde_json-b69a92aa1055d696.rlib --extern time=/checkout/obj/build/tmp/distcheck/build/bootstrap/debug/deps/libtime-1b8611857d71d3f6.rlib --extern toml=/checkout/obj/build/tmp/distcheck/build/bootstrap/debug/deps/libtoml-477f7006fd660090.rlib -Wrust_2018_idioms -Wunused_lifetimes -Wsemicolon_in_expressions_from_macros -Dwarnings` (signal: 11, SIGSEGV: invalid memory reference)
error: build failed
failed to run: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/obj/build/tmp/distcheck/src/bootstrap/Cargo.toml --locked --frozen
Build completed unsuccessfully in 0:00:49
Build completed unsuccessfully in 0:00:49
make: *** [check] Error 1
Makefile:42: recipe for target 'check' failed

command did not execute successfully: "make" "check"
expected success, got: exit status: 2

