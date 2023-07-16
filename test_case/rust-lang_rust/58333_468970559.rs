plain
travis_time:end:0994a356:start=1551568900230894653,finish=1551568974936165779,duration=74705271126
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:06:35]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:06:49] error[E0599]: no variant named `Promoted` found for type `mir::Place<'_>` in the current scope
[00:06:49]     --> src/librustc/mir/mod.rs:3315:17
[00:06:49]      |
[00:06:49] 1898 | pub enum Place<'tcx> {
[00:06:49]      | -------------------- variant `Promoted` not found here
[00:06:49] ...
[00:06:49] 3315 |         (Place::Promoted)(promoted),
[00:06:49]      | 
[00:06:49]     ::: src/librustc/macros.rs:425:21
[00:06:49]      |
[00:06:49]      |
[00:06:49] 425  |                     $variant ( $($variant_arg),* ) => {
[00:06:49] 
[00:06:49] error[E0599]: no variant named `Static` found for type `mir::Place<'_>` in the current scope
[00:06:49]     --> src/librustc/mir/mod.rs:3314:17
[00:06:49]      |
[00:06:49]      |
[00:06:49] 1898 | pub enum Place<'tcx> {
[00:06:49]      | -------------------- variant `Static` not found here
[00:06:49] ...
[00:06:49] 3314 |         (Place::Static)(statik),
[00:06:49]      | 
[00:06:49]     ::: src/librustc/macros.rs:425:21
[00:06:49]      |
[00:06:49]      |
[00:06:49] 425  |                     $variant ( $($variant_arg),* ) => {
[00:06:49] 
[00:06:49] error[E0599]: no variant named `Local` found for type `mir::Place<'_>` in the current scope
[00:06:49]     --> src/librustc/mir/mod.rs:3313:17
[00:06:49]      |
[00:06:49]      |
[00:06:49] 1898 | pub enum Place<'tcx> {
[00:06:49]      | -------------------- variant `Local` not found here
[00:06:49] ...
[00:06:49] 3313 |         (Place::Local)(local),
[00:06:49]      | 
[00:06:49]     ::: src/librustc/macros.rs:425:21
[00:06:49]      |
[00:06:49]      |
[00:06:49] 425  |                     $variant ( $($variant_arg),* ) => {
[00:06:49]      |
[00:06:49]      = help: did you mean `local`?
[00:06:49] 
[00:06:49] error[E0599]: no variant named `Promoted` found for type `mir::Place<'_>` in the current scope
[00:06:49] error[E0599]: no variant named `Promoted` found for type `mir::Place<'_>` in the current scope
[00:06:49]     --> src/librustc/mir/mod.rs:3315:17
[00:06:49]      |
[00:06:49] 1898 | pub enum Place<'tcx> {
[00:06:49]      | -------------------- variant `Promoted` not found here
[00:06:49] ...
[00:06:49] 3315 |         (Place::Promoted)(promoted),
[00:06:49]      | 
[00:06:49]     ::: src/librustc/macros.rs:426:37
[00:06:49]      |
[00:06:49]      |
[00:06:49] 426  |                         Ok($variant (
[00:06:49]      |                                     - variant not found in `mir::Place<'_>`
[00:06:49] error[E0599]: no variant named `Static` found for type `mir::Place<'_>` in the current scope
[00:06:49]     --> src/librustc/mir/mod.rs:3314:17
[00:06:49]      |
[00:06:49] 1898 | pub enum Place<'tcx> {
[00:06:49] 1898 | pub enum Place<'tcx> {
[00:06:49]      | -------------------- variant `Static` not found here
[00:06:49] ...
[00:06:49] 3314 |         (Place::Static)(statik),
[00:06:49]      | 
[00:06:49]     ::: src/librustc/macros.rs:426:37
[00:06:49]      |
[00:06:49]      |
[00:06:49] 426  |                         Ok($variant (
[00:06:49]      |                                     - variant not found in `mir::Place<'_>`
[00:06:49] error[E0599]: no variant named `Local` found for type `mir::Place<'_>` in the current scope
[00:06:49]     --> src/librustc/mir/mod.rs:3313:17
[00:06:49]      |
[00:06:49] 1898 | pub enum Place<'tcx> {
[00:06:49] 1898 | pub enum Place<'tcx> {
[00:06:49]      | -------------------- variant `Local` not found here
[00:06:49] ...
[00:06:49] 3313 |         (Place::Local)(local),
[00:06:49]      | 
[00:06:49]     ::: src/librustc/macros.rs:426:37
[00:06:49]      |
[00:06:49]      |
[00:06:49] 426  |                         Ok($variant (
[00:06:49]      |                                     - variant not found in `mir::Place<'_>`
[00:06:49]      = help: did you mean `local`?
[00:06:49] 
[00:06:49] error[E0599]: no variant named `Promoted` found for type `mir::Place<'_>` in the current scope
[00:06:49]     --> src/librustc/mir/mod.rs:3315:17
[00:06:49]     --> src/librustc/mir/mod.rs:3315:17
[00:06:49]      |
[00:06:49] 1898 | pub enum Place<'tcx> {
[00:06:49]      | -------------------- variant `Promoted` not found here
[00:06:49] ...
[00:06:49] 3315 |         (Place::Promoted)(promoted),
[00:06:49]      | 
[00:06:49]     ::: src/librustc/macros.rs:481:21
[00:06:49]      |
[00:06:49]      |
[00:06:49] 481  |                     $variant ( $($variant_arg),* ) => {
[00:06:49] 
[00:06:49] error[E0599]: no variant named `Static` found for type `mir::Place<'_>` in the current scope
[00:06:49]     --> src/librustc/mir/mod.rs:3314:17
[00:06:49]      |
[00:06:49]      |
[00:06:49] 1898 | pub enum Place<'tcx> {
[00:06:49]      | -------------------- variant `Static` not found here
[00:06:49] ...
[00:06:49] 3314 |         (Place::Static)(statik),
[00:06:49]      | 
[00:06:49]     ::: src/librustc/macros.rs:481:21
[00:06:49]      |
[00:06:49]      |
[00:06:49] 481  |                     $variant ( $($variant_arg),* ) => {
[00:06:49] 
[00:06:49] error[E0599]: no variant named `Local` found for type `mir::Place<'_>` in the current scope
[00:06:49]     --> src/librustc/mir/mod.rs:3313:17
[00:06:49]      |
[00:06:49]      |
[00:06:49] 1898 | pub enum Place<'tcx> {
[00:06:49]      | -------------------- variant `Local` not found here
[00:06:49] ...
[00:06:49] 3313 |         (Place::Local)(local),
[00:06:49]      | 
[00:06:49]     ::: src/librustc/macros.rs:481:21
[00:06:49]      |
[00:06:49]      |
[00:06:49] 481  |                     $variant ( $($variant_arg),* ) => {
[00:06:49]      |
[00:06:49]      = help: did you mean `local`?
[00:06:49] 
[00:07:08] error: aborting due to 9 previous errors
---
30764 ./src/llvm-project/llgo
29516 ./src/llvm-project/llgo/third_party
27124 ./src/llvm-project/llgo/third_party/gofrontend
26652 ./src/llvm-emscripten/test/Transforms
26168 ./obj/agnosticReports/': No such file or directory
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:193d5800
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: ‘/home/travis/Library/Logs/DiagnosticReports’: No such file or directory
travis_time:end:193d5800:start=1551569413069362973,finish=1551569413074075558,duration=4712585
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00c6dbea
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2116fab8
travis_time:start:2116fab8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No s
