
---- [ui (nll)] ui/const-eval/enum_discr.rs stdout ----
	

executing "/<<BUILDDIR>>/rustc-1.27.1+dfsg1/build/s390x-unknown-linux-gnu/stage2/bin/rustc" "/<<BUILDDIR>>/rustc-1.27.1+dfsg1/src/test/ui/const-eval/enum_discr.rs" "-L" "/<<BUILDDIR>>/rustc-1.27.1+dfsg1/build/s390x-unknown-linux-gnu/test/ui" "--target=s390x-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/<<BUILDDIR>>/rustc-1.27.1+dfsg1/build/s390x-unknown-linux-gnu/test/ui/const-eval/enum_discr.stage2-s390x-unknown-linux-gnu" "-Zborrowck=mir" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/<<BUILDDIR>>/rustc-1.27.1+dfsg1/build/s390x-unknown-linux-gnu/native/rust-test-helpers" "-L" "/<<BUILDDIR>>/rustc-1.27.1+dfsg1/build/s390x-unknown-linux-gnu/test/ui/const-eval/enum_discr.stage2-s390x-unknown-linux-gnu.aux" "-A" "unused"
------stdout------------------------------

------stderr------------------------------
thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', libcore/option.rs:335:21
note: Run with `RUST_BACKTRACE=1` for a backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.27.1 running on s390x-unknown-linux-gnu

note: compiler flags: -Z ui-testing -Z borrowck=mir -Z two-phase-borrows -Z unstable-options -C prefer-dynamic -C rpath

...

failures:
    [ui (nll)] ui/const-eval/enum_discr.rs
    [ui (nll)] ui/const-eval/promoted_errors.rs
    [ui (nll)] ui/dropck/dropck-eyepatch-extern-crate.rs
    [ui (nll)] ui/dropck/dropck-eyepatch-reorder.rs
    [ui (nll)] ui/dropck/dropck-eyepatch.rs
    [ui (nll)] ui/loops-reject-duplicate-labels-2.rs
    [ui (nll)] ui/loops-reject-duplicate-labels.rs
    [ui (nll)] ui/nll/issue-43058.rs
    [ui (nll)] ui/span/dropck_vec_cycle_checked.rs
    [ui (nll)] ui/span/issue28498-reject-lifetime-param.rs
    [ui (nll)] ui/span/issue28498-reject-passed-to-fn.rs
    [ui (nll)] ui/span/issue28498-reject-trait-bound.rs
