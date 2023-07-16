plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:0bc724a0
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:09:33]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:09:48] error[E0599]: no variant named `Promoted` found for type `mir::Place<'_>` in the current scope
[00:09:48]     --> src/librustc/mir/mod.rs:3315:17
[00:09:48]      |
[00:09:48] 1898 | pub enum Place<'tcx> {
[00:09:48]      | -------------------- variant `Promoted` not found here
[00:09:48] ...
[00:09:48] 3315 |         (Place::Promoted)(promoted),
[00:09:48]      | 
[00:09:48]     ::: src/librustc/macros.rs:425:21
[00:09:48]      |
[00:09:48]      |
[00:09:48] 425  |                     $variant ( $($variant_arg),* ) => {
[00:09:48] 
[00:09:49] error[E0599]: no variant named `Static` found for type `mir::Place<'_>` in the current scope
[00:09:49]     --> src/librustc/mir/mod.rs:3314:17
[00:09:49]      |
[00:09:49]      |
[00:09:49] 1898 | pub enum Place<'tcx> {
[00:09:49]      | -------------------- variant `Static` not found here
[00:09:49] ...
[00:09:49] 3314 |         (Place::Static)(statik),
[00:09:49]      | 
[00:09:49]     ::: src/librustc/macros.rs:425:21
[00:09:49]      |
[00:09:49]      |
[00:09:49] 425  |                     $variant ( $($variant_arg),* ) => {
[00:09:49] 
[00:09:49] error[E0599]: no variant named `Local` found for type `mir::Place<'_>` in the current scope
[00:09:49]     --> src/librustc/mir/mod.rs:3313:17
[00:09:49]      |
[00:09:49]      |
[00:09:49] 1898 | pub enum Place<'tcx> {
[00:09:49]      | -------------------- variant `Local` not found here
[00:09:49] ...
[00:09:49] 3313 |         (Place::Local)(local),
[00:09:49]      | 
[00:09:49]     ::: src/librustc/macros.rs:425:21
[00:09:49]      |
[00:09:49]      |
[00:09:49] 425  |                     $variant ( $($variant_arg),* ) => {
[00:09:49]      |
[00:09:49]      = help: did you mean `local`?
[00:09:49] 
[00:09:49] error[E0599]: no variant named `Promoted` found for type `mir::Place<'_>` in the current scope
[00:09:49] error[E0599]: no variant named `Promoted` found for type `mir::Place<'_>` in the current scope
[00:09:49]     --> src/librustc/mir/mod.rs:3315:17
[00:09:49]      |
[00:09:49] 1898 | pub enum Place<'tcx> {
[00:09:49]      | -------------------- variant `Promoted` not found here
[00:09:49] ...
[00:09:49] 3315 |         (Place::Promoted)(promoted),
[00:09:49]      | 
[00:09:49]     ::: src/librustc/macros.rs:426:37
[00:09:49]      |
[00:09:49]      |
[00:09:49] 426  |                         Ok($variant (
[00:09:49]      |                                     - variant not found in `mir::Place<'_>`
[00:09:49] error[E0599]: no variant named `Static` found for type `mir::Place<'_>` in the current scope
[00:09:49]     --> src/librustc/mir/mod.rs:3314:17
[00:09:49]      |
[00:09:49] 1898 | pub enum Place<'tcx> {
[00:09:49] 1898 | pub enum Place<'tcx> {
[00:09:49]      | -------------------- variant `Static` not found here
[00:09:49] ...
[00:09:49] 3314 |         (Place::Static)(statik),
[00:09:49]      | 
[00:09:49]     ::: src/librustc/macros.rs:426:37
[00:09:49]      |
[00:09:49]      |
[00:09:49] 426  |                         Ok($variant (
[00:09:49]      |                                     - variant not found in `mir::Place<'_>`
[00:09:49] error[E0599]: no variant named `Local` found for type `mir::Place<'_>` in the current scope
[00:09:49]     --> src/librustc/mir/mod.rs:3313:17
[00:09:49]      |
[00:09:49] 1898 | pub enum Place<'tcx> {
[00:09:49] 1898 | pub enum Place<'tcx> {
[00:09:49]      | -------------------- variant `Local` not found here
[00:09:49] ...
[00:09:49] 3313 |         (Place::Local)(local),
[00:09:49]      | 
[00:09:49]     ::: src/librustc/macros.rs:426:37
[00:09:49]      |
[00:09:49]      |
[00:09:49] 426  |                         Ok($variant (
[00:09:49]      |                                     - variant not found in `mir::Place<'_>`
[00:09:49]      = help: did you mean `local`?
[00:09:49] 
[00:09:49] error[E0599]: no variant named `Promoted` found for type `mir::Place<'_>` in the current scope
[00:09:49]     --> src/librustc/mir/mod.rs:3315:17
[00:09:49]     --> src/librustc/mir/mod.rs:3315:17
[00:09:49]      |
[00:09:49] 1898 | pub enum Place<'tcx> {
[00:09:49]      | -------------------- variant `Promoted` not found here
[00:09:49] ...
[00:09:49] 3315 |         (Place::Promoted)(promoted),
[00:09:49]      | 
[00:09:49]     ::: src/librustc/macros.rs:481:21
[00:09:49]      |
[00:09:49]      |
[00:09:49] 481  |                     $variant ( $($variant_arg),* ) => {
[00:09:49] 
[00:09:49] error[E0599]: no variant named `Static` found for type `mir::Place<'_>` in the current scope
[00:09:49]     --> src/librustc/mir/mod.rs:3314:17
[00:09:49]      |
[00:09:49]      |
[00:09:49] 1898 | pub enum Place<'tcx> {
[00:09:49]      | -------------------- variant `Static` not found here
[00:09:49] ...
[00:09:49] 3314 |         (Place::Static)(statik),
[00:09:49]      | 
[00:09:49]     ::: src/librustc/macros.rs:481:21
[00:09:49]      |
[00:09:49]      |
[00:09:49] 481  |                     $variant ( $($variant_arg),* ) => {
[00:09:49] 
[00:09:49] error[E0599]: no variant named `Local` found for type `mir::Place<'_>` in the current scope
[00:09:49]     --> src/librustc/mir/mod.rs:3313:17
[00:09:49]      |
[00:09:49]      |
[00:09:49] 1898 | pub enum Place<'tcx> {
[00:09:49]      | -------------------- variant `Local` not found here
[00:09:49] ...
[00:09:49] 3313 |         (Place::Local)(local),
[00:09:49]      | 
[00:09:49]     ::: src/librustc/macros.rs:481:21
[00:09:49]      |
[00:09:49]      |
[00:09:49] 481  |                     $variant ( $($variant_arg),* ) => {
[00:09:49]      |
[00:09:49]      = help: did you mean `local`?
[00:09:49] 
[00:10:08] error: aborting due to 9 previous errors
---
travis_time:end:2089da00:start=1551569553792249873,finish=1551569553797928516,duration=5678643
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04f77494
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:13b42cb6
travis_time:start:13b42cb6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:094e26d6
$ dmesg | grep -i kill
