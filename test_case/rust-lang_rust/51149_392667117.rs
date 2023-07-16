plain
[00:27:45]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:27:57]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:33:07]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:33:07]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:33:09] error: `...` range patterns are deprecated
[00:33:09]    --> librustc_mir/hair/pattern/check_match.rs:467:18
[00:33:09]     |
[00:33:09] 467 |                 2...LIMIT => {
[00:33:09]     |                  ^^^ help: use `..=` for an inclusive range
[00:33:09]     |
[00:33:09]     = note: `-D inclusive-range-pattern-syntax` implied by `-D warnings`
[00:33:09]     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:33:09] 
[00:33:25] error: aborting due to previous error
[00:33:25] 
[00:33:25] error: Could not compile `rustc_mir`.
[00:33:25] error: Could not compile `rustc_mir`.
[00:33:25] 
[00:33:25] Caused by:
[00:33:25]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_mir librustc_mir/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=3 -C metadata=8b64d19076a61749 -C extra-filename=-8b64d19076a61749 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-f21bfea456e2feba.so --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-b0021385c43a16dd.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-3762ade15a64029b.so --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-58601def030aa1ca.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-e7bbb7d6e0541d97.so --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-9dea40d5c994cba1.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-70b92be3dfddcce2.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-2d5dc4c4f204cfc5.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-2d5dc4c4f204cfc5.rlib --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-6bb876a3fc9c03dd.so --extern log_settings=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog_settings-a74e6a1a0a4b163f.rlib --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-575f47f158b62d9a.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-faa7967fee325699.rlib --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-f5ce6dd7626a50ea.so --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-270afc7a968c2570.rlib --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-19b2a6eb2f045d57.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-ea1f3bc77d3b46e4/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-53de06a86865b430/out` (exit code: 101)
Tue, 29 May 2018 06:32:17 GMT
travis_time:end:0e5c2de0:start=1527575537915252787,finish=1527575537974335007,duration=59082220

The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
