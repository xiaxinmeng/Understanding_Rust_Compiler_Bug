plain
ruby 2.4.1p111 (2017-03-22 revision 58053) [x86_64-linux]
travis_fold:end:system_info

Network availability confirmed.
Setting APT mirror in /etc/apt/sources.list: http://archive.ubuntu.com/ubuntu/
Installing APT Packages
travis_time:start:0016fad0
$ travis_apt_get_update
travis_time:end:0016fad0:start=1536692160827439199,finish=1536692170566048302,duration=9738609103
travis_time:end:0016fad0:start=1536692160827439199,finish=1536692170566048302,duration=9738609103
travis_time:start:05da6c28
$ sudo -E apt-get -yq --no-install-suggests --no-install-recommends $(travis_apt_get_options) install gdb
Building dependency tree...
Reading state information...
Suggested packages:
  gdb-doc gdbserver
---
[00:44:01]     |                                                              ^^^^^^^^^^^^^^^^^^^^^ cannot be resolved, ignoring
[00:44:01]     |
[00:44:01]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:44:01] 
[00:44:01] error: internal compiler error: cannot load crate `std` after `freeze_crate_numbers` was called by query engine creation (currently `TyCtxt::create_and_enter`)
[00:44:01] thread '<unnamed>' panicked at 'Box<Any>', librustc_errors/lib.rs:525:9
[00:44:01] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:44:01] 
[00:44:01] error: Unrecognized option: 'crate-version'
[00:44:01] error: Unrecognized option: 'crate-version'
[00:44:01] 
[00:44:01] error: Could not document `std`.
[00524b19ad7b.rmeta --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-58f8274ab338318c.rmeta --extern rustc_asan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_asan-a35cca948f6d387d.rmeta --extern rustc_lsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_lsan-12d57d04c7ffffb2.rmeta --extern rustc_msan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_msan-f8433a02a5ab1950.rmeta --extern rustc_tsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_tsan-4204718e4beedd4e.rmeta --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libunwind-60e8daa9ec5ead85.rmeta` (exit code: 1)
[00:44:01] 
[00:44:01] 
[00:44:01] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--no-deps" "-p" "alloc" "-p" "core" "-p" "std"
[00:44:01] 
[00:44:01] 
[00:44:01] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc
[00:44:01] Build completed unsuccessfully in 0:04:51
[00:44:01] Build completed unsuccessfully in 0:04:51
[00:44:01] Makefile:28: recipe for target 'all' failed
[00:44:01] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00b1b430
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
