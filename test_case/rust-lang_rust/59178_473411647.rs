plain
travis_time:end:007c3fe0:start=1552677138731564164,finish=1552677141169742334,duration=2438178170
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:06:23]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:06:36]    Compiling synstructure v0.10.1
[00:06:57]    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
[00:07:51]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:08:23] error[E0277]: the trait bound `mir::interpret::value::ConstValue<'_>: ty::context::Lift<'_>` is not satisfied
[00:08:23]     --> src/librustc/ty/print/pretty.rs:1399:25
[00:08:23] 1394 | / macro_rules! forward_display_to_print {
[00:08:23] 1394 | / macro_rules! forward_display_to_print {
[00:08:23] 1395 | |     ($($ty:ty),+) => {
[00:08:23] 1396 | |         $(impl fmt::Display for $ty {
[00:08:23] 1397 | |             fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
[00:08:23] 1398 | |                 ty::tls::with(|tcx| {
[00:08:23] 1399 | |                     tcx.lift(self)
[00:08:23]      | |                         ^^^^ the trait `ty::context::Lift<'_>` is not implemented for `mir::interpret::value::ConstValue<'_>`
[00:08:23] 1406 | |     };
[00:08:23] 1407 | | }
[00:08:23]      | |_- in this expansion of `forward_display_to_print!`
[00:08:23] 1408 | 
[00:08:23] 1408 | 
[00:08:23] 1409 | / macro_rules! define_print_and_forward_display {
[00:08:23] 1410 | |     (($self:ident, $cx:ident): $($ty:ty $print:block)+) => {
[00:08:23] 1411 | |         $(impl<'gcx: 'tcx, 'tcx, P: PrettyPrinter<'gcx, 'tcx>> Print<'gcx, 'tcx, P> for $ty {
[00:08:23] 1412 | |             type Output = P;
[00:08:23] ...    |
[00:08:23] 1424 | |         forward_display_to_print!($($ty),+);
[00:08:23] 1425 | |     };
[00:08:23] 1426 | | }
[00:08:23]      | |_- in this expansion of `define_print_and_forward_display!`
[00:08:23] ...
[00:08:23] ...
[00:08:23] 1457 | / define_print_and_forward_display! {
[00:08:23] 1458 | |     (self, cx):
[00:08:23] 1459 | |
[00:08:23] 1460 | |     &'tcx ty::List<Ty<'tcx>> {
[00:08:23] 1607 | |     }
[00:08:23] 1608 | | }
[00:08:23]      | |_- in this macro invocation
[00:08:23] 
[00:08:23] 
[00:08:23] error[E0277]: the trait bound `ty::sty::Const<'_>: ty::context::Lift<'_>` is not satisfied
[00:08:23]     --> src/librustc/ty/print/pretty.rs:1399:25
[00:08:23] 1394 | / macro_rules! forward_display_to_print {
[00:08:23] 1394 | / macro_rules! forward_display_to_print {
[00:08:23] 1395 | |     ($($ty:ty),+) => {
[00:08:23] 1396 | |         $(impl fmt::Display for $ty {
[00:08:23] 1397 | |             fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
[00:08:23] 1398 | |                 ty::tls::with(|tcx| {
[00:08:23] 1399 | |                     tcx.lift(self)
[00:08:23]      | |                         ^^^^ the trait `ty::context::Lift<'_>` is not implemented for `ty::sty::Const<'_>`
[00:08:23] 1406 | |     };
[00:08:23] 1407 | | }
[00:08:23]      | |_- in this expansion of `forward_display_to_print!`
[00:08:23] 1408 | 
[00:08:23] 1408 | 
[00:08:23] 1409 | / macro_rules! define_print_and_forward_display {
[00:08:23] 1410 | |     (($self:ident, $cx:ident): $($ty:ty $print:block)+) => {
[00:08:23] 1411 | |         $(impl<'gcx: 'tcx, 'tcx, P: PrettyPrinter<'gcx, 'tcx>> Print<'gcx, 'tcx, P> for $ty {
[00:08:23] 1412 | |             type Output = P;
[00:08:23] ...    |
[00:08:23] 1424 | |         forward_display_to_print!($($ty),+);
[00:08:23] 1425 | |     };
[00:08:23] 1426 | | }
[00:08:23]      | |_- in this expansion of `define_print_and_forward_display!`
[00:08:23] ...
[00:08:23] ...
[00:08:23] 1457 | / define_print_and_forward_display! {
[00:08:23] 1458 | |     (self, cx):
[00:08:23] 1459 | |
[00:08:23] 1460 | |     &'tcx ty::List<Ty<'tcx>> {
[00:08:23] 1607 | |     }
[00:08:23] 1608 | | }
[00:08:23]      | |_- in this macro invocation
[00:08:23]      |
[00:08:23]      |
[00:08:23]      = help: the following implementations were found:
[00:08:23]                <&'a ty::sty::Const<'a> as ty::context::Lift<'tcx>>
[00:08:27] error: aborting due to 2 previous errors
[00:08:27] 
[00:08:27] For more information about this error, try `rustc --explain E0277`.
[00:08:27] error: Could not compile `rustc`.
---
travis_time:end:252507ec:start=1552677660801931999,finish=1552677660806864477,duration=4932478
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0237e372
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:30cc9006
travis_time:start:30cc9006
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02a25daa
$ dmesg | grep -i kill
