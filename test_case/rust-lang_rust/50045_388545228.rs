plain
[00:25:52]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:26:05]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:30:57]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:30:57]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:30:59] thread 'main' panicked at 'librustc/hir/map/mod.rs:914: expected expr, found block {
[00:30:59]     ::std::ops::Try::from_ok({
[00:30:59]                                  let mut file =
[00:30:59]                                      match ::std::ops::Try::into_result(pretty::create_dump_file(infcx.tcx,
[00:30:59]                                                                                                  "regioncx.dot",
[00:30:59]                                                                                                  "nll",
[00:30:59]                                                                                                  &0,
[00:30:59]                                                                                                  source))
[00:30:59]                                          {
[00:30:59]                                          {
[00:30:59]                                          ::std::result::Result::Err(err) =>
[00:30:59]                                              #[allow(unreachable_code)]
[00:30:59]                                              break
[00:30:59]                                                  ::std::ops::Try::from_error(::std::convert::From::from(err))
[00:30:59]                                                  ,
[00:30:59]                                          ::std::result::Result::Ok(val) =>
[00:30:59]                                              #[allow(unreachable_code)]
[00:30:59]                                              val,
[00:30:59]                                      };
[00:30:59]                                  match ::std::ops::Try::into_result(regioncx.dump_graphviz(&mut file))
[00:30:59]                                      {
[00:30:59]                                      ::std::result::Result::Err(err) =>
[00:30:59]                                          #[allow(unreachable_code)]
[00:30:59]                                          break
[00:30:59]                                              ::std::ops::Try::from_error(::std::convert::From::from(err))
[00:30:59]                                              ,
[00:30:59]                                      ::std::result::Result::Ok(val) =>
[00:30:59]                                          #[allow(unreachable_code)]
[00:30:59]                                          val,
[00:30:59]                              })
[00:30:59]                              })
[00:30:59] } (id=132252)', librustc/session/mod.rs:1288:26
[00:30:59] 
[00:30:59] error: internal compiler error: unexpected panic
[00:30:59] 
[00:30:59] 
[00:30:59] note: the compiler unexpectedly panicked. this is a bug.
[00:30:59] 
[00:30:59] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:30:59] note: rustc 1.27.0-dev running on x86_64-unknown-linux-gnu
[00:30:59] 
[00:30:59] 
[00:30:59] note: compiler flags: -Z force-unstable-if-unmarked -C prefer-dynamic -C opt-level=3 -C prefer-dynamic -C debug-assertions=y -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type dylib
[00:30:59] 
[00:30:59] note: some of the compiler flags provided by cargo are hidden
[00:30:59] error: Could not compile `rustc_mir`.
[00:30:59] 
[00:30:59] Caused by:
[00:30:59] Caused by:
[00:30:59]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_mir librustc_mir/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=3 -C metadata=fb1e714f8b62af65 -C extra-filename=-fb1e714f8b62af65 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-4bd92253de06945f.rlib --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-e4198b380764cbac.so --extern log_settings=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog_settings-dc728a812bba07b6.rlib --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-5fa4b8500b7192a8.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-cff695d4a4682a45.so --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-8616faa2bf885029.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-c485a7f1a48de680.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-cf6fe4bd350080f4.so --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-7fb6d9dea690f7ef.so --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-eb65188a952b4d73.rlib --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-36905ab7cba6c774.rlib --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-09bdd144093f3a08.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-09bdd144093f3a08.rlib --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-ce13477f28b2521d.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-0df30ab4137d926b.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-3089210d8394aa26/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-f7531eecb12e933e/out` (exit code: 101)
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:0d174155:start=1526120299596124663,finish=1526120299602996205,duration=6871542
travis_fold:end:after_failure.3
