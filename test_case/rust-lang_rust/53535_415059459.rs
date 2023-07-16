plain
[00:14:13]    Compiling rustc_metadata_utils v0.0.0 (file:///checkout/src/librustc_metadata_utils)
[00:14:13]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:14:13]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:14:13]    Compiling rustc_allocator v0.0.0 (file:///checkout/src/librustc_allocator)
[00:14:26] error[E0599]: no method named `write_value_to_ptr` found for type `&mut interpret::eval_context::EvalContext<'a, 'mir, 'tcx, interpret::const_eval::CompileTimeEvaluator>` in the current scope
[00:14:26]    --> librustc_mir/interpret/const_eval.rs:350:21
[00:14:26]     |
[00:14:26] 350 |                 ecx.write_value_to_ptr(args[0].value, ptr.into(), dest_align, src_ty).unwrap();
[00:14:26] 
[00:14:26] error[E0609]: no field `value` on type `interpret::operand::OpTy<'tcx>`
[00:14:26]    --> librustc_mir/interpret/const_eval.rs:350:48
[00:14:26]     |
[00:14:26]     |
[00:14:26] 350 |                 ecx.write_value_to_ptr(args[0].value, ptr.into(), dest_align, src_ty).unwrap();
[00:14:26]     |
[00:14:26]     |
[00:14:26]     = note: available fields are: `op`, `layout`
[00:14:27] error: aborting due to 2 previous errors
[00:14:27] 
[00:14:27] Some errors occurred: E0599, E0609.
[00:14:27] For more information about an error, try `rustc --explain E0599`.
[00:14:27] For more information about an error, try `rustc --explain E0599`.
[00:14:27] error: Could not compile `rustc_mir`.
[00:14:27] 
[00:14:27] Caused by:
[00:14:27]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_mir librustc_mir/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=0d31b3db998f4b07 -C extra-filename=-0d31b3db998f4b07 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-b8f9e6fb5ae336d7.so --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-3907cba388d41ef0.rlib --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-8246be02936c9b1b.rlib --extern either=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libeither-0a515e87c8afea9e.rlib --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-45ae4394366d07fd.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-b6c566856a1e65b9.rlib --extern log_settings=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog_settings-f501f9d595863bbf.rlib --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-c6e8cf18dad58451.rlib --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-3a367afe0d047d15.so --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-165c205e2819b15f.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-8b624a6d6082b2ff.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-89eed8215142aadd.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-20eb47b9c402fee3.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-8c9bc9ee6cc9592f.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-8c9bc9ee6cc9592f.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-64bbe8e4870170a3.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-3b51f50aecba154c.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-6c2cab36846647d7/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-3c585aa15bfc4e69/out` (exit code: 1)
time:start:296021cb
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:296021cb:start=1534949376934774870,finish=1534949376941892415,duration=7117545
---
travis_time:end:08a06fd8:start=1534949376948587839,finish=1534949376955788214,duration=7200375
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:32141708
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1b03a9b5
travis_time:start:1b03a9b5
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers ||
