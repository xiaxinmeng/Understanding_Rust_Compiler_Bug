plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:09aa16d8
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:19:04]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:23:59]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[00:23:59]    Compiling rustc_allocator v0.0.0 (/checkout/src/librustc_allocator)
[00:23:59]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:24:11] error[E0599]: no method named `len` found for type `rustc_data_structures::work_queue::WorkQueue<rustc::mir::BasicBlock>` in the current scope
[00:24:11]    |
[00:24:11]    |
[00:24:11] 98 |     debug_assert!(dirty_queue.len() == body.basic_blocks().len(),
[00:24:11] 
[00:24:15] error: aborting due to previous error
[00:24:15] 
[00:24:15] For more information about this error, try `rustc --explain E0599`.
---
travis_time:end:1e38342a:start=1561243411948741422,finish=1561243411955191662,duration=6450240
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0aeb5ab3
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$COREtravis_time:start:16a82d86
travis_time:end:16a82d86:start=1561243411990408017,finish=1561243412004394914,duration=13986897
travis_fold:end:after_failure.6

Done. Your build exited with 1.
