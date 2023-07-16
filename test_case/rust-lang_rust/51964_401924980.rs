plain
[00:07:51]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:08:06]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:13:35]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:13:35]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:13:35] error: this file contains an un-closed delimiter
[00:13:35]    --> librustc_mir/build/mod.rs:804:12
[00:13:35]     |
[00:13:35] 804 | mod scope;
[00:13:35]     |
[00:13:35] help: did you mean to close this delimiter?
[00:13:35]    --> librustc_mir/build/mod.rs:605:46
[00:13:35]     |
[00:13:35]     |
[00:13:35] 605 | impl<'a, 'gcx, 'tcx> Builder<'a, 'gcx, 'tcx> {
[00:13:35] 
[00:13:35] 
[00:13:35] error: expected one of `::` or `:`, found `)`
[00:13:35]    --> librustc_mir/build/mod.rs:757:31
[00:13:35]     |
[00:13:35] 757 |     fn get_unit_temp(&mut self) -> Place<'tcx> {
[00:13:35]     |                               ^ expected one of `::` or `:` here
[00:13:35] 
[00:13:35] error: expected one of `::` or `:`, found `)`
[00:13:35]    --> librustc_mir/build/mod.rs:770:30
[00:13:35]     |
[00:13:35] 770 |     fn return_block(&mut self) -> BasicBlock {
[00:13:35]     |                              ^ expected one of `::` or `:` here
[00:13:35] 
[00:13:35] error: expected one of `::` or `:`, found `)`
[00:13:35]    --> librustc_mir/build/mod.rs:781:35
[00:13:35]     |
[00:13:35] 781 |     fn unreachable_block(&mut self) -> BasicBlock {
[00:13:35]     |                                   ^ expected one of `::` or `:` here
[00:13:35] 
[00:13:35] error: expected one of `const`, `crate`, `default`, `extern`, `fn`, `pub`, `type`, `unsafe`, or `}`, found `mod`
[00:13:35]    --> librustc_mir/build/mod.rs:798:1
[00:13:35] 791 | }
[00:13:35] 791 | }
[00:13:35]     |  - expected one of 9 possible tokens here
[00:13:35] ...
[00:13:35] 798 | mod block;
[00:13:35]     | ^^^ unexpected token
[00:13:35] 
[00:13:35] error: expected one of `const`, `crate`, `default`, `extern`, `fn`, `pub`, `type`, `unsafe`, or `}`, found `mod`
[00:13:35]    --> librustc_mir/build/mod.rs:799:1
[00:13:35]     |
[00:13:35] 798 | mod block;
[00:13:35]     |           - expected one of 9 possible tokens here
[00:13:35] 799 | mod cfg;
[00:13:35]     | ^^^ unexpected token
[00:13:35] 
[00:13:35] error: expected one of `const`, `crate`, `default`, `extern`, `fn`, `pub`, `type`, `unsafe`, or `}`, found `mod`
[00:13:35]    --> librustc_mir/build/mod.rs:800:1
[00:13:35]     |
[00:13:35] 799 | mod cfg;
[00:13:35]     |         - expected one of 9 possible tokens here
[00:13:35] 800 | mod expr;
[00:13:35]     | ^^^ unexpected token
[00:13:35] 
[00:13:35] error: expected one of `const`, `crate`, `default`, `extern`, `fn`, `pub`, `type`, `unsafe`, or `}`, found `mod`
[00:13:35]    --> librustc_mir/build/mod.rs:801:1
[00:13:35]     |
[00:13:35] 800 | mod expr;
[00:13:35]     |          - expected one of 9 possible tokens here
[00:13:35] 801 | mod into;
[00:13:35]     | ^^^ unexpected token
[00:13:35] 
[00:13:35] error: expected one of `const`, `crate`, `default`, `extern`, `fn`, `pub`, `type`, `unsafe`, or `}`, found `mod`
[00:13:35]    --> librustc_mir/build/mod.rs:802:1
[00:13:35]     |
[00:13:35] 801 | mod into;
[00:13:35]     |          - expected one of 9 possible tokens here
[00:13:35] 802 | mod matches;
[00:13:35]     | ^^^ unexpected token
[00:13:35] 
[00:13:35] error: expected one of `const`, `crate`, `default`, `extern`, `fn`, `pub`, `type`, `unsafe`, or `}`, found `mod`
[00:13:35]    --> librustc_mir/build/mod.rs:803:1
[00:13:35]     |
[00:13:35] 802 | mod matches;
[00:13:35]     |             - expected one of 9 possible tokens here
[00:13:35] 803 | mod misc;
[00:13:35]     | ^^^ unexpected token
[00:13:35] 
[00:13:35] error: expected one of `const`, `crate`, `default`, `extern`, `fn`, `pub`, `type`, `unsafe`, or `}`, found `mod`
[00:13:35]    --> librustc_mir/build/mod.rs:804:1
[00:13:35]     |
[00:13:35] 803 | mod misc;
[00:13:35]     |          - expected one of 9 possible tokens here
[00:13:35] 804 | mod scope;
[00:13:35]     | ^^^ unexpected token
[00:13:37] error[E0433]: failed to resolve. Use of undeclared type or module `scope`
[00:13:37]    --> librustc_mir/build/mod.rs:260:17
[00:13:37]     |
[00:13:37]     |
[00:13:37] 260 |     scopes: Vec<scope::Scope<'tcx>>,
[00:13:37]     |                 ^^^^^ Use of undeclared type or module `scope`
[00:13:37] error[E0433]: failed to resolve. Use of undeclared type or module `scope`
[00:13:37]    --> librustc_mir/build/mod.rs:271:27
[00:13:37]     |
[00:13:37]     |
[00:13:37] 271 |     breakable_scopes: Vec<scope::BreakableScope<'tcx>>,
[00:13:37]     |                           ^^^^^ Use of undeclared type or module `scope`
[00:13:37] error[E0433]: failed to resolve. Use of undeclared type or module `matches`
[00:13:37]    --> librustc_mir/build/mod.rs:735:55
[00:13:37]     |
[00:13:37]     |
[00:13:37] 735 |                                                       matches::ArmHasGuard(false),
[00:13:37]     |                                                       ^^^^^^^ Use of undeclared type or module `matches`
[00:13:37] 
[00:13:37] error[E0434]: can't capture dynamic environment in a fn item
[00:13:37]    --> librustc_mir/build/mod.rs:758:15
[00:13:37]     |
[00:13:37] 758 |         match self.unit_temp {
[00:13:37]     |
[00:13:37]     |
[00:13:37]     = help: use the `|| { ... }` closure form instead
[00:13:37] 
[00:13:37] error[E0434]: can't capture dynamic environment in a fn item
[00:13:37]    --> librustc_mir/build/mod.rs:761:26
[00:13:37]     |
[00:13:37] 761 |                 let ty = self.hir.unit_ty();
[00:13:37]     |
[00:13:37]     |
[00:13:37]     = help: use the `|| { ... }` closure form instead
[00:13:37] 
[00:13:37] error[E0434]: can't capture dynamic environment in a fn item
[00:13:37]    --> librustc_mir/build/mod.rs:762:31
[00:13:37]     |
[00:13:37] 762 |                 let fn_span = self.fn_span;
[00:13:37]     |
[00:13:37]     |
[00:13:37]     = help: use the `|| { ... }` closure form instead
[00:13:37] 
[00:13:37] error[E0434]: can't capture dynamic environment in a fn item
[00:13:37]    --> librustc_mir/build/mod.rs:763:27
[00:13:37]     |
[00:13:37] 763 |                 let tmp = self.temp(ty, fn_span);
[00:13:37]     |
[00:13:37]     |
[00:13:37]     = help: use the `|| { ... }` closure form instead
[00:13:37] 
[00:13:37] error[E0434]: can't capture dynamic environment in a fn item
[00:13:37]    --> librustc_mir/build/mod.rs:764:17
[00:13:37]     |
[00:13:37] 764 |                 self.unit_temp = Some(tmp.clone());
[00:13:37]     |
[00:13:37]     |
[00:13:37]     = help: use the `|| { ... }` closure form instead
[00:13:37] 
[00:13:37] error[E0434]: can't capture dynamic environment in a fn item
[00:13:37]    --> librustc_mir/build/mod.rs:771:15
[00:13:37]     |
[00:13:37] 771 |         match self.cached_return_block {
[00:13:37]     |
[00:13:37]     |
[00:13:37]     = help: use the `|| { ... }` closure form instead
[00:13:37] 
[00:13:37] error[E0434]: can't capture dynamic environment in a fn item
[00:13:37]    --> librustc_mir/build/mod.rs:774:26
[00:13:37]     |
[00:13:37] 774 |                 let rb = self.cfg.start_new_block();
[00:13:37]     |
[00:13:37]     |
[00:13:37]     = help: use the `|| { ... }` closure form instead
[00:13:37] 
[00:13:37] error[E0434]: can't capture dynamic environment in a fn item
[00:13:37]    --> librustc_mir/build/mod.rs:775:17
[00:13:37]     |
[00:13:37] 775 |                 self.cached_return_block = Some(rb);
[00:13:37]     |
[00:13:37]     |
[00:13:37]     = help: use the `|| { ... }` closure form instead
[00:13:37] 
[00:13:37] error[E0434]: can't capture dynamic environment in a fn item
[00:13:37]    --> librustc_mir/build/mod.rs:782:15
[00:13:37]     |
[00:13:37] 782 |         match self.cached_unreachable_block {
[00:13:37]     |
[00:13:37]     |
[00:13:37]     = help: use the `|| { ... }` closure form instead
[00:13:37] 
[00:13:37] error[E0434]: can't capture dynamic environment in a fn item
[00:13:37]    --> librustc_mir/build/mod.rs:785:26
[00:13:37]     |
[00:13:37] 785 |                 let ub = self.cfg.start_new_block();
[00:13:37]     |
[00:13:37]     |
[00:13:37]     = help: use the `|| { ... }` closure form instead
[00:13:37] 
[00:13:37] error[E0434]: can't capture dynamic environment in a fn item
[00:13:37]    --> librustc_mir/build/mod.rs:786:17
[00:13:37]     |
[00:13:37] 786 |                 self.cached_unreachable_block = Some(ub);
[00:13:37]     |
[00:13:37]     |
[00:13:37]     = help: use the `|| { ... }` closure form instead
[00:13:37] 
[00:13:38] error[E0261]: use of undeclared lifetime name `'tcx`
[00:13:38]    --> librustc_mir/build/mod.rs:757:42
[00:13:38]     |
[00:13:38] 757 |     fn get_unit_temp(&mut self) -> Place<'tcx> {
[00:13:38]     |                                          ^^^^ undeclared lifetime
[00:13:38] error: aborting due to 26 previous errors
[00:13:38] 
[00:13:38] Some errors occurred: E0261, E0433, E0434.
[00:13:38] For more information about an error, try `rustc --explain E0261`.
[00:13:38] For more information about an error, try `rustc --explain E0261`.
[00:13:38] error: Could not compile `rustc_mir`.
[00:13:38] 
[00:13:38] Caused by:
[00:13:38]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_mir librustc_mir/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=8fc603afb8ea9b13 -C extra-filename=-8fc603afb8ea9b13 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-d45628fe21047b42.so --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-d8b3f1986e621085.rlib --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-09e9dbd1ef48ffa5.rlib --extern either=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libeither-b51deb005c05dd3b.rlib --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-6020508f01da724d.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-023d781fbd65d983.rlib --extern log_settings=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog_settings-9c861d36e123bec8.rlib --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-2e546cbdf217aece.rlib --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-1dddb0fa9d8a512f.so --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-be9737b074a8dae0.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-25582ce13d3618ea.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-e036f8f5b9204e52.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-066098b54e835a1f.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-05774896829b5d87.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-05774896829b5d87.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-a1b02e0d020520ac.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-f36f6cb494d373be.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-a59a4023b81a89b2/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-ea5907e223517cf2/out` (exit code: 101)
[00:15:13] error: build failed
[00:15:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:15:13] expected success, got: exit code: 101
[00:15:13] expected success, got: exit code: 101
[00:15:13] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:15:13] travis_fold:end:stage0-rustc

[00:15:13] travis_time:end:stage0-rustc:start=1530562869979106655,finish=1530563435511843621,duration=565532736966


[00:15:13] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:15:13] Build completed unsuccessfully in 0:09:36
[00:15:13] Makefile:28: recipe for target 'all' failed
[00:15:13] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0083fad6
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:17bfccc2:start=1530563436130972155,finish=1530563436139496370,duration=8524215
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0529960a
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:060c6871
$ dmesg | grep -i kill
