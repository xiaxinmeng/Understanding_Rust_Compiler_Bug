plain
[00:00:47] configure: rust.quiet-tests     := True
---
[00:06:58] error[E0599]: no method named `as_str` found for type `&Term` in the current scope
[00:06:58]    --> libproc_macro/lib.rs:736:14
[00:06:58]     |
[00:06:58] 736 |         self.as_str().fmt(f)
[00:06:58]     |              ^^^^^^
[00:06:58]
[00:06:59] error[E0599]: no method named `as_str` found for type `Term` in the current scope
[00:06:59]    --> libproc_macro/quote.rs:186:40
[00:06:59]     |
[00:06:59] 186 |         quote!(::Term::new((quote self.as_str()), (quote self.span())))
[00:06:59]     |                                        ^^^^^^
[00:06:59]     |
[00:06:59]    ::: libproc_macro/lib.rs:686:1
[00:06:59]     |
[00:06:59] 686 | pub struct Term {
[00:06:59]     | --------------- method `as_str` not found for this
[00:06:59]
[00:06:59] error: aborting due to 2 previous errors
[00:06:59]
[00:06:59] For more information about this error, try `rustc --explain E0599`.
[00:06:59] error: Could not compile `proc_macro`.
[00:06:59]
[00:06:59] Caused by:
[00:06:59]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name proc_macro libproc_macro/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=6e4119b5ec8457a3 -C extra-filename=-6e4119b5ec8457a3 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-93cb1ddd29ab61a4.so --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-d3b6fcf798f7d22a.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-609e2421d03f9c9a.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-9f3518d56a01456f.so` (exit code: 101)
[00:06:59] warning: build failed, waiting for other jobs to finish...
[00:07:09] error: build failed
[00:07:09] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:07:09] expected success, got: exit code: 101
[00:07:09] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1085:9
[00:07:09] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:07:09] travis_fold:end:stage0-rustc
[00:07:09] travis_time:end:stage0-rustc:start=1523311840034999165,finish=1523311976282252106,duration=136247252941
[00:07:09] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:07:09] Build completed unsuccessfully in 0:02:29
[00:07:09] Makefile:28: recipe for target 'all' failed
[00:07:09] make: *** [all] Error 1
---
121664 ./obj/build/bootstrap/debug/incremental/bootstrap-351vorei3hhuv/s-ezyvpuqw47-r07ced-179sb4yv5t1yk
---
10012 ./src/llvm/test/MC/AMDGPU
9648 ./src/llvm/test/MC/Disassembler/AMDGPU
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:040871be:start=1523311976827397498,finish=1523311976834701564,duration=7304066
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:1aa653d0
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'co
