plain
[00:07:57] 127 |             _ => Lrc::new(vec![]),
[00:07:57]     |                           ^^^^^^ expected reference, found struct `std::vec::Vec`
[00:07:57]     |
[00:07:57]     = note: expected type `&rustc::ty::Slice<rustc::traits::Clause<'_>>`
[00:07:57]                found type `std::vec::Vec<_>`
[00:07:57]     = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:07:57]
[00:07:57] error[E0308]: mismatched types
[00:07:57]    --> librustc_traits/lowering.rs:131:17
[00:07:57]     |
[00:07:57] 131 |                 program_clauses_for_associated_type_value(tcx, def_id)
[00:07:57]     |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected reference, found struct `std::vec::Vec`
[00:07:57]     |
[00:07:57]     = note: expected type `std::sync::Arc<&'tcx rustc::ty::Slice<rustc::traits::Clause<'tcx>>>`
[00:07:57]                found type `std::sync::Arc<std::vec::Vec<rustc::traits::Clause<'_>>>`
[00:07:57]
[00:07:57] error[E0308]: mismatched types
[00:07:57]    --> librustc_traits/lowering.rs:133:26
[00:07:57]     |
[00:07:57] 133 |                 Lrc::new(vec![])
[00:07:57]     |                          ^^^^^^ expected reference, found struct `std::vec::Vec`
[00:07:57]     |
[00:07:57]     = note: expected type `&rustc::ty::Slice<rustc::traits::Clause<'_>>`
[00:07:57]                found type `std::vec::Vec<_>`
[00:07:57]     = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:07:57]
[00:07:57] error[E0277]: the trait bound `&rustc::ty::Slice<rustc::traits::Goal<'_>>: std::iter::FromIterator<_>` is not satisfied
[00:07:57]    --> librustc_traits/lowering.rs:292:67
[00:07:57]     |
[00:07:57] 292 |         hypotheses: where_clauses.into_iter().map(|wc| wc.into()).collect(),
[00:07:57]     |                                                                   ^^^^^^^ a collection of type `&rustc::ty::Slice<rustc::traits::Goal<'_>>` cannot be built from an iterator over elements of type `_`
[00:07:57]     |
[00:07:57]     = help: the trait `std::iter::FromIterator<_>` is not implemented for `&rustc::ty::Slice<rustc::traits::Goal<'_>>`
---
[00:07:57]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_traits librustc_traits/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,metadata -C prefer-dynamic -C debug-assertions=off -C overflow-checks=on -C metadata=514151e0c679572e -C extra-filename=-514151e0c679572e --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -C incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/incremental -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-1dffe97d0651361b.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-c43c75e6fa38c52c.rmeta --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-75f2b25c19b6a07d.rmeta --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-e790766762a27d5b.rmeta --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-5726b7c8b62bc86c.rmeta --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-ee064adeca477563.rmeta --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-faf6f68107bf25e3.rmeta -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-8b35e3c2ea935fab/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-63734d0048644b22/out` (exit code: 101)
[00:07:57] warning: build failed, waiting for other jobs to finish...
[00:08:23] error: build failed
[00:08:23] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:08:23] expected success, got: exit code: 101
[00:08:23] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1085:9
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:0148e153:start=1523635449151315637,finish=1523635449161561919,duration=10246282
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:07b53a78
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:07b53a78:start=1523635449169036891,finish=1523635449176700836,duration=7663945
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:043eef50
$ dmesg | grep -i kill
[   11.337887] init: failsafe main process (1093) killed by TERM signal
