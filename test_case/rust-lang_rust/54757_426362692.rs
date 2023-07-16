plain
[00:23:42]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:23:46]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:25:16]    Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
[00:25:25]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:27:09] error: internal compiler error: librustc_mir/borrow_check/nll/universal_regions.rs:754: cannot convert `ReScope(Remainder { block: ItemLocalId(1359), first_statement_index: 0})` to a region vid
[00:27:09] thread 'main' panicked at 'Box<Any>', librustc_errors/lib.rs:599:9
[00:27:09] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:27:10] error: aborting due to previous error
[00:27:10] 
[00:27:10] 
[00:27:10] 
[00:27:10] note: the compiler unexpectedly panicked. this is a bug.
[00:27:10] 
[00:27:10] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:27:10] 
[00:27:10] note: rustc 1.31.0-dev running on x86_64-unknown-linux-gnu
[00:27:10] 
[00:27:10] note: compiler flags: -Z force-unstable-if-unmarked -C prefer-dynamic -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type dylib
[00:27:10] note: some of the compiler flags provided by cargo are hidden
[00:27:10] 
[00:27:10] error: Could not compile `rustc`.
[00:27:10] 
[00:27:10] 
[00:27:10] To learn more, run the command again with --verbose.
[00:27:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:27:10] expected success, got: exit code: 101
[00:27:10] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1112:9
[00:27:10] travis_fold:end:stage1-rustc

[00:27:10] travis_time:end:stage1-rustc:start=1538501328026176391,finish=1538501597202990906,duration=269176814515

---
travis_time:end:2bef507a:start=1538501597933763060,finish=1538501597938549283,duration=4786223
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1b4dd171
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
