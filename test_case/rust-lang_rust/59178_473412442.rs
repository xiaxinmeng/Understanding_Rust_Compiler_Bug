plain
[00:04:29]     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:04:52]    Compiling synstructure v0.10.1
[00:04:58]     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:05:07]    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
[00:05:29] error[E0277]: the trait bound `mir::interpret::value::ConstValue<'_>: ty::context::Lift<'_>` is not satisfied
[00:05:29]     --> src/librustc/ty/print/pretty.rs:1399:25
[00:05:29] 1394 | / macro_rules! forward_display_to_print {
[00:05:29] 1394 | / macro_rules! forward_display_to_print {
[00:05:29] 1395 | |     ($($ty:ty),+) => {
[00:05:29] 1396 | |         $(impl fmt::Display for $ty {
[00:05:29] 1397 | |             fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
[00:05:29] 1398 | |                 ty::tls::with(|tcx| {
[00:05:29] 1399 | |                     tcx.lift(self)
[00:05:29]      | |                         ^^^^ the trait `ty::context::Lift<'_>` is not implemented for `mir::interpret::value::ConstValue<'_>`
[00:05:29] 1406 | |     };
[00:05:29] 1407 | | }
[00:05:29]      | |_- in this expansion of `forward_display_to_print!`
[00:05:29] 1408 | 
[00:05:29] 1408 | 
[00:05:29] 1409 | / macro_rules! define_print_and_forward_display {
[00:05:29] 1410 | |     (($self:ident, $cx:ident): $($ty:ty $print:block)+) => {
[00:05:29] 1411 | |         $(impl<'gcx: 'tcx, 'tcx, P: PrettyPrinter<'gcx, 'tcx>> Print<'gcx, 'tcx, P> for $ty {
[00:05:29] 1412 | |             type Output = P;
[00:05:29] ...    |
[00:05:29] 1424 | |         forward_display_to_print!($($ty),+);
[00:05:29] 1425 | |     };
[00:05:29] 1426 | | }
[00:05:29]      | |_- in this expansion of `define_print_and_forward_display!`
[00:05:29] ...
[00:05:29] ...
[00:05:29] 1457 | / define_print_and_forward_display! {
[00:05:29] 1458 | |     (self, cx):
[00:05:29] 1459 | |
[00:05:29] 1460 | |     &'tcx ty::List<Ty<'tcx>> {
[00:05:29] 1607 | |     }
[00:05:29] 1608 | | }
[00:05:29]      | |_- in this macro invocation
[00:05:29] 
[00:05:29] 
[00:05:29] error[E0277]: the trait bound `ty::sty::Const<'_>: ty::context::Lift<'_>` is not satisfied
[00:05:29]     --> src/librustc/ty/print/pretty.rs:1399:25
[00:05:29] 1394 | / macro_rules! forward_display_to_print {
[00:05:29] 1394 | / macro_rules! forward_display_to_print {
[00:05:29] 1395 | |     ($($ty:ty),+) => {
[00:05:29] 1396 | |         $(impl fmt::Display for $ty {
[00:05:29] 1397 | |             fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
[00:05:29] 1398 | |                 ty::tls::with(|tcx| {
[00:05:29] 1399 | |                     tcx.lift(self)
[00:05:29]      | |                         ^^^^ the trait `ty::context::Lift<'_>` is not implemented for `ty::sty::Const<'_>`
[00:05:29] 1406 | |     };
[00:05:29] 1407 | | }
[00:05:29]      | |_- in this expansion of `forward_display_to_print!`
[00:05:29] 1408 | 
[00:05:29] 1408 | 
[00:05:29] 1409 | / macro_rules! define_print_and_forward_display {
[00:05:29] 1410 | |     (($self:ident, $cx:ident): $($ty:ty $print:block)+) => {
[00:05:29] 1411 | |         $(impl<'gcx: 'tcx, 'tcx, P: PrettyPrinter<'gcx, 'tcx>> Print<'gcx, 'tcx, P> for $ty {
[00:05:29] 1412 | |             type Output = P;
[00:05:29] ...    |
[00:05:29] 1424 | |         forward_display_to_print!($($ty),+);
[00:05:29] 1425 | |     };
[00:05:29] 1426 | | }
[00:05:29]      | |_- in this expansion of `define_print_and_forward_display!`
[00:05:29] ...
[00:05:29] ...
[00:05:29] 1457 | / define_print_and_forward_display! {
[00:05:29] 1458 | |     (self, cx):
[00:05:29] 1459 | |
[00:05:29] 1460 | |     &'tcx ty::List<Ty<'tcx>> {
[00:05:29] 1607 | |     }
[00:05:29] 1608 | | }
[00:05:29]      | |_- in this macro invocation
[00:05:29]      |
[00:05:29]      |
[00:05:29]      = help: the following implementations were found:
[00:05:29]                <&'a ty::sty::Const<'a> as ty::context::Lift<'tcx>>
[00:05:33] error: aborting due to 2 previous errors
[00:05:33] 
[00:05:33] For more information about this error, try `rustc --explain E0277`.
[00:05:33] error: Could not compile `rustc`.
---
travis_time:end:06a575a2:start=1552677830732770050,finish=1552677830738623554,duration=5853504
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2027ee70
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:103ab790
travis_time:start:103ab790
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0065031c
$ dmesg | grep -i kill
