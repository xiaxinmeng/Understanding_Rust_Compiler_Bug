plain
[00:07:40]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:13:06]    Compiling rustc_metadata_utils v0.0.0 (file:///checkout/src/librustc_metadata_utils)
[00:13:06]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:13:06]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:13:06] error: expected one of `.`, `;`, `?`, `}`, or an operator, found `i`
[00:13:06]    --> librustc_mir/interpret/eval_context.rs:672:48
[00:13:06]     |
[00:13:06] 672 |         self.tcx.const_eval(param_env.and(gid))i
[00:13:06]     |                                                ^ expected one of `.`, `;`, `?`, `}`, or an operator here
[00:13:06] 
[00:13:06] error: expected one of `,` or `>`, found `'tcx`
[00:13:06]     --> librustc_mir/interpret/memory.rs:1058:23
[00:13:06]      |
[00:13:06] 1058 | where M: Machine<'mir 'tcx>
[00:13:06]      |                       ^^^^ expected one of `,` or `>` here
[00:13:06] error: aborting due to 2 previous errors
[00:13:06] 
[00:13:06] error: Could not compile `rustc_mir`.
[00:13:06] 
[00:13:06] 
[00:13:06] Caused by:
[00:13:06]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_mir librustc_mir/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=0d31b3db998f4b07 -C extra-filename=-0d31b3db998f4b07 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-b8f9e6fb5ae336d7.so --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-3907cba388d41ef0.rlib --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-8246be02936c9b1b.rlib --extern either=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libeither-0a515e87c8afea9e.rlib --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-45ae4394366d07fd.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-b6c566856a1e65b9.rlib --extern log_settings=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog_settings-f501f9d595863bbf.rlib --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-c6e8cf18dad58451.rlib --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-3a367afe0d047d15.so --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-165c205e2819b15f.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-8b624a6d6082b2ff.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-89eed8215142aadd.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-20eb47b9c402fee3.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-8c9bc9ee6cc9592f.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-8c9bc9ee6cc9592f.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-64bbe8e4870170a3.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-3b51f50aecba154c.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-6c2cab36846647d7/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-3c585aa15bfc4e69/out` (exit code: 1)
3211364 .
1374788 ./obj
1374752 ./obj/build
1058724 ./src
---
151200 ./src/tools/clang
149124 ./src/llvm-emscripten/test
148688 ./obj/build/bootstrap/debug/incremental
134256 ./obj/build/bootstrap/debug/incremental/bootstrap-1v3ifugz4t07z
134252 ./obj/build/bootstrap/debug/incremental/bootstrap-1v3ifugz4t07z/s-f43j6hi2j8-wlpiy4-1iplbejydu42a
108992 ./src/llvm/test/CodeGen
103868 ./src/tools/lldb
98952 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
93756 ./src/tools/clang/test
