plain
travis_time:end:05feb51c:start=1542539309924564381,finish=1542539312065556592,duration=2140992211
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:50:46] .................................................................................................... 100/5022
[00:50:49] .................................................................................................... 200/5022
[00:50:52] .............................ii............................................ii...................ii.. 300/5022
[00:50:55] ..............................................................................................iii... 400/5022
[00:50:58] .....iiiiiiii.iii............................iii...........................................i........ 500/5022
[00:51:05] .................................................................................................... 700/5022
[00:51:11] .................................................................................i...........i...... 800/5022
[00:51:15] .................................................................................................... 900/5022
[00:51:18] iiiii..................ii.iiii...................................................................... 1000/5022
---
[00:51:54] .................................................................................................... 2200/5022
[00:51:59] .................................................................................................... 2300/5022
[00:52:03] .................................................................................................... 2400/5022
[00:52:06] .................................................................................................... 2500/5022
[00:52:10] .................................................................................iiiiiiiii.......... 2600/5022
[00:52:17] ...............................................ii................................................... 2800/5022
[00:52:20] .................................................................................................... 2900/5022
[00:52:24] .................................................................................................... 3000/5022
[00:52:27] ..........................................i......................................................... 3100/5022
---
[00:53:57] .................................................................................................... 300/2884
[00:54:09] .................................................................................................... 400/2884
[00:54:18] .................................................................................................... 500/2884
[00:54:29] .................................................................................................... 600/2884
[00:54:44] ............................FF...................................................................... 700/2884
[00:54:55] .................................................................................................... 800/2884
[00:55:05] ...............................F.................................................................... 900/2884
[00:55:33] .................................................................................................... 1100/2884
[00:55:42] .................................................................................................... 1200/2884
[00:55:51] .................................................................................................... 1300/2884
[00:56:04] ..............................................................................i..................... 1400/2884
---
[00:59:38] failures:
[00:59:38] 
[00:59:38] ---- [run-pass] run-pass/drop/dynamic-drop.rs#lexical stdout ----
[00:59:38] 
[00:59:38] error in revision `lexical`: test run failed!
[00:59:38] status: signal: 4
[00:59:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/drop/dynamic-drop.lexical/a"
[00:59:38] ------------------------------------------
[00:59:38] 
[00:59:38] ------------------------------------------
[00:59:38] stderr:
[00:59:38] stderr:
[00:59:38] ------------------------------------------
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/d panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-paspass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'missing free: [true, false]', /checkout/src/test/run-pass/drop/dynamic-drop.rs:42:13
[00:59:38] thread 'main' panicked at 'missing free: [true, false]', /checkout/src/test/run-pass/drop/dynamic-drop.rs:42:13
[00:59:38] stack backtrace:
[00:59:38]    0:     0x7f2e2a31fe63 - std::sys::unix::backtrace::tracing::imp::unwind_backtrace::h85e3a547bfeb3499
[00:59:38]    1:     0x7f2e2a316f68 - std::sys_common::backtrace::_print::h345e53235c3c4217
[00:59:38]    2:     0x7f2e2a31c5b0 - std::panicking::default_hook::{{closure}}::h51afb1efda32ec18
[00:59:38]    3:     0x7f2e2a31c287 - std::panicking::default_hook::hd81881f8c9effee8
[00:59:38]    4:     0x7f2e2a31cd8a - std::panicking::rust_panic_with_hook::h020097611d33156d
[00:59:38]    5:     0x7f2e2a31c949 - std::panicking::continue_panic_fmt::hf8b038a62bdee60d
[00:59:38]    6:     0x7f2e2a31c8ad - std::panicking::begin_panic_fmt::h570839826960e210
[00:59:38]    7:     0x56000da84cb0 - core::ptr::drop_in_place::hd87fdedc8297dcef
[00:59:38]    8:     0x56000da8b97a - dynamic_drop::main::hd5140cf70bc8e103
[00:59:38]    9:     0x56000da8dc12 - std::rt::lang_start::{{closure}}::h4b98e9bdbf1e2a0a
[00:59:38]   10:     0x7f2e2a31c7e2 - std::panicking::try::do_call::h6dff7fd2ae4225ad
[00:59:38]   11:     0x7f2e2a3389e9 - __rust_maybe_catch_panic
[00:59:38]   12:     0x7f2e2a31d2a6 - std::rt::lang_start_internal::h6ed29bf54717c5ec
[00:59:38]   13:     0x56000da8b9b3 - main
[00:59:38]   14:     0x7f2e29ceeckout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:82:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59:13
[00:59:38] thread 'main' panicked at 'Box<Any>', /checkout/src/test/run-pass/drop/dynamic-drop.rs:59d_backtrace::h85e3a547bfeb3499
[00:59:38]    1:     0x7f972960ff68 - std::sys_common::backtrace::_print::h345e53235c3c4217
[00:59:38]    2:     0x7f97296155b0 - std::panicking::default_hook::{{closure}}::h51afb1efda32ec18
[00:59:38]    3:     0x7f9729615287 - std::panicking::default_hook::hd81881f8c9effee8
[00:59:38]    4:     0x7f9729615d8a - std::panicking::rust_panic_with_hook::h020097611d33156d
[00:59:38]    5:     0x7f9729615949 - std::panicking::continue_panic_fmt::hf8b038a62bdee60d
[00:59:38]    6:     0x7f97296158ad - std::panicking::begin_panic_fmt::h570839826960e210
[00:59:38]    7:     0x5612918d6cb0 - core::ptr::drop_in_place::hd87fdedc8297dcef
[00:59:38]    8:     0x5612918dd97a - dynamic_drop::main::hd5140cf70bc8e103
[00:59:38]    9:     0x5612918dfc12 - std::rt::lang_start::{{closure}}::h4b98e9bdbf1e2a0a
[00:59:38]   10:     0x7f97296157e2 - std::panicking::try::do_call::h6dff7fd2ae4225ad
[00:59:38]   11:     0x7f97296319e9 - __rust_maybe_catch_panic
[00:59:38]   12:     0x7f97296162a6 - std::rt::lang_start_internal::h6ed29bf54717c5ec
[00:59:38]   13:     0x5612918dd9b3 - main
[00:59:38]   14:     0x7f9728fde82f - __libc_start_main
[00:59:38]   15:     0x5612918d67f8 - _start
[00:59:38]   16:                0x0 - <unknown>
[00:59:38] thread panicked while panicking. aborting.
[00:59:38] ------------------------------------------
[00:59:38] 
[00:59:38] thread '[run-pass] run-pass/drop/dynamic-drop.rs#nll' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9
[00:59:38] 
---
[00:59:38] test result: FAILED. 2871 passed; 3 failed; 10 ignored; 0 measured; 0 filtered out
[00:59:38] 
[00:59:38] 
[00:59:38] 
[00:59:38] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/srcn-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib
55716 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release
52788 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release/deps
51388 ./obj/build/x86_64-unknown-linux-gnu/stage0/bin
50132 ./src/llvm/test/CodeGen/X86
---
37004 ./obj/build/x86_64-unknown-linux-gnu/doc/core/arch
36516 ./.git/modules/src/libcompiler_builtins/modules
36000 ./.git/modules/src/libcompiler_builtins/modules/compiler-rt
35640 ./src/tools/clang/lib
35544 ./.git/modules/src/libcompiler_builtins/ng::begin_panic_fmt::h570839826960e210 () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-4b5dc58ca7b0caa7.so
#3  0x000056000da84cb1 in core::ptr::drop_in_place::hd87fdedc8297dcef ()
#4  0x000056000da8b97b in dynamic_drop::main::hd5140cf70bc8e103 ()
#5  0x000056000da8dc13 in std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h4b98e9bdbf1e2a0a ()
#6  0x00007f2e2a31c7e3 in std::panicking::try::do_call::h6dff7fd2ae4225ad () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-4b5dc58ca7b0caa7.so
#7  0x00007f2e2a3389ea in __rust_maybe_catch_panic () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-4b5dc58ca7b0caa7.so
#8  0x00007f2e2a31d2a7 in std::rt::lang_start_internal::h6ed29bf54717c5ec () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-4b5dc58ca7b0caa7.so
#9  0x000056000da8b9b4 in main ()
travis_fold:start:crashlog
travis_fold:start:crashlog
obj/cores/core.32234.!checkout!obj!build!x86_64-unknown-linux-gnu!test!run-pass!drop!dynamic-drop.nll!a
[New LWP 32234]
warning: Could not load shared library symbols for 6 libraries, e.g. /lib/x86_64-linux-gnu/libgcc_s.so.1.
Use the "info sharedlibrary" command to see the complete listing.
Do you need "set solib-search-path" or "set sysroot"?
Core was generated by `/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/drop/dynamic-drop.nl'.
Program terminated with signal SIGILL, Illegal instruction.
#0  0x00007f9729615c5e in std::panicking::rust_panic_with_hook::h020097611d33156d () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-4b5dc58ca7b0caa7.so
#0  0x00007f9729615c5e in std::panicking::rust_panic_with_hook::h020097611d33156d () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-4b5dc58ca7b0caa7.so
#1  0x00007f972961594a in std::panicking::continue_panic_fmt::hf8b038a62bdee60d () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-4b5dc58ca7b0caa7.so
#2  0x00007f97296158ae in std::panicking::begin_panic_fmt::h570839826960e210 () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-4b5dc58ca7b0caa7.so
#3  0x00005612918d6cb1 in core::ptr::drop_in_place::hd87fdedc8297dcef ()
#4  0x00005612918dd97b in dynamic_drop::main::hd5140cf70bc8e103 ()
#5  0x00005612918dfc13 in std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h4b98e9bdbf1e2a0a ()
#6  0x00007f97296157e3 in std::panicking::try::do_call::h6dff7fd2ae4225ad () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-4b5dc58ca7b0caa7.so
#7  0x00007f97296319ea in __rust_maybe_catch_panic () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-4b5dc58ca7b0caa7.so
#8  0x00007f97296162a7 in std::rt::lang_start_internal::h6ed29bf54717c5ec () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-4b5dc58ca7b0caa7.so
