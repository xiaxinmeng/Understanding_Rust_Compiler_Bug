
---- [ui] ui/nll/issue-43058.rs stdout ----
	

executing "/<<BUILDDIR>>/rustc-1.27.1+dfsg1/build/s390x-unknown-linux-gnu/stage2/bin/rustc" "/<<BUILDDIR>>/rustc-1.27.1+dfsg1/src/test/ui/nll/issue-43058.rs" "-L" "/<<BUILDDIR>>/rustc-1.27.1+dfsg1/build/s390x-unknown-linux-gnu/test/ui" "--target=s390x-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/<<BUILDDIR>>/rustc-1.27.1+dfsg1/build/s390x-unknown-linux-gnu/test/ui/nll/issue-43058.stage2-s390x-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/<<BUILDDIR>>/rustc-1.27.1+dfsg1/build/s390x-unknown-linux-gnu/native/rust-test-helpers" "-L" "/<<BUILDDIR>>/rustc-1.27.1+dfsg1/build/s390x-unknown-linux-gnu/test/ui/nll/issue-43058.stage2-s390x-unknown-linux-gnu.aux" "-A" "unused"
------stdout------------------------------

------stderr------------------------------
thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', libcore/option.rs:335:21
note: Run with `RUST_BACKTRACE=1` for a backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.27.1 running on s390x-unknown-linux-gnu

note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
