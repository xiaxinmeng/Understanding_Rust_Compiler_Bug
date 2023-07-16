plain
travis_time:end:01988a04:start=1541093667286994812,finish=1541093721096541729,duration=53809546917
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:53:19] .................................................................................................... 100/4983
[00:53:22] .................................................................................................... 200/4983
[00:53:25] ...........................................................................................ii....... 300/4983
[00:53:27] .........................................................................................iii........ 400/4983
[00:53:30] iiiiiiii.iii...........................iii...........................................i...........i.. 500/4983
[00:53:38] .................................................................................................... 700/4983
[00:53:44] ..................................................................i...........i..................... 800/4983
[00:53:47] ....................................................................................iiiii........... 900/4983
[00:53:51] .................................................................................................... 1000/4983
---
[00:54:28] .................................................................................................... 2200/4983
[00:54:32] .................................................................................................... 2300/4983
[00:54:36] .................................................................................................... 2400/4983
[00:54:40] .................................................................................................... 2500/4983
[00:54:44] ...................................................................iiiiiiiii........................ 2600/4983
[00:54:51] ..................ii................................................................................ 2800/4983
[00:54:54] .................................................................................................... 2900/4983
[00:54:58] .................................................................................................... 3000/4983
[00:55:00] .............i...................................................................................... 3100/4983
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:08:47] 
[01:08:47] running 115 tests
[01:08:50] i..ii...iii..iii.....i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..ii 100/115
[01:08:51] .i....iiii.....
[01:08:51] 
[01:08:51]  finished in 3.684
[01:08:51] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:09:06] 
[01:09:06] running 118 tests
[01:09:31] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:09:35] ......iii.i.....ii
[01:09:35] 
[01:09:35]  finished in 29.329
[01:09:35] travis_fold:end:test_debuginfo

---
[01:28:13] travis_fold:start:test_stage1-std
travis_time:start:test_stage1-std
Testing std stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:28:13]    Compiling std v0.0.0 (/checkout/src/libstd)
[01:28:23] error[E0277]: can't compare `&realstd::vec::Vec<u8>` with `[{integer}; 2]`
[01:28:23]      |
[01:28:23]      |
[01:28:23] 1101 |         assert_eq!(writer.get_ref(), [0, 1]);
[01:28:23]      |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `&realstd::vec::Vec<u8> == [{integer}; 2]`
[01:28:23]      |
[01:28:23]      = help: the trait `core::cmp::PartialEq<[{integer}; 2]>` is not implemented for `&realstd::vec::Vec<u8>`
[01:28:23] 
[01:28:23] 
[01:28:23] error[E0277]: can't compare `&realstd::vec::Vec<u8>` with `[{integer}; 2]`
[01:28:23]      |
[01:28:23]      |
[01:28:23] 1105 |         assert_eq!(writer.get_ref(), [0, 1]);
[01:28:23]      |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `&realstd::vec::Vec<u8> == [{integer}; 2]`
[01:28:23]      |
[01:28:23]      = help: the trait `core::cmp::PartialEq<[{integer}; 2]>` is not implemented for `&realstd::vec::Vec<u8>`
[01:28:23] 
[01:28:23] 
[01:28:23] error[E0277]: can't compare `&realstd::vec::Vec<u8>` with `[{integer}; 2]`
[01:28:23]      |
[01:28:23]      |
[01:28:23] 1109 |         assert_eq!(writer.get_ref(), [0, 1]);
[01:28:23]      |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `&realstd::vec::Vec<u8> == [{integer}; 2]`
[01:28:23]      |
[01:28:23]      = help: the trait `core::cmp::PartialEq<[{integer}; 2]>` is not implemented for `&realstd::vec::Vec<u8>`
[01:28:23] 
[01:28:23] 
[01:28:23] error[E0277]: can't compare `&realstd::vec::Vec<u8>` with `[{integer}; 4]`
[01:28:23]      |
[01:28:23]      |
[01:28:23] 1113 |         assert_eq!(writer.get_ref(), [0, 1, 2, 3]);
[01:28:23]      |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `&realstd::vec::Vec<u8> == [{integer}; 4]`
[01:28:23]      |
[01:28:23]      = help: the trait `core::cmp::PartialEq<[{integer}; 4]>` is not implemented for `&realstd::vec::Vec<u8>`
[01:28:23] 
[01:28:23] 
[01:28:23] error[E0277]: can't compare `&realstd::vec::Vec<u8>` with `[{integer}; 4]`
[01:28:23]      |
[01:28:23]      |
[01:28:23] 1118 |         assert_eq!(writer.get_ref(), [0, 1, 2, 3]);
[01:28:23]      |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `&realstd::vec::Vec<u8> == [{integer}; 4]`
[01:28:23]      |
[01:28:23]      = help: the trait `core::cmp::PartialEq<[{integer}; 4]>` is not implemented for `&realstd::vec::Vec<u8>`
[01:28:23] 
[01:28:23] 
[01:28:23] error[E0277]: can't compare `&realstd::vec::Vec<u8>` with `[{integer}; 6]`
[01:28:23]      |
[01:28:23]      |
[01:28:23] 1122 |         assert_eq!(writer.get_ref(), [0, 1, 2, 3, 4, 5]);
[01:28:23]      |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `&realstd::vec::Vec<u8> == [{integer}; 6]`
[01:28:23]      |
[01:28:23]      = help: the trait `core::cmp::PartialEq<[{integer}; 6]>` is not implemented for `&realstd::vec::Vec<u8>`
[01:28:23] 
[01:28:23] 
[01:28:23] error[E0277]: can't compare `&realstd::vec::Vec<u8>` with `[{integer}; 9]`
[01:28:23]      |
[01:28:23]      |
[01:28:23] 1126 |         assert_eq!(writer.get_ref(), [0, 1, 2, 3, 4, 5, 6, 7, 8]);
[01:28:23]      |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `&realstd::vec::Vec<u8> == [{integer}; 9]`
[01:28:23]      |
[01:28:23]      = help: the trait `core::cmp::PartialEq<[{integer}; 9]>` is not implemented for `&realstd::vec::Vec<u8>`
[01:28:23] 
[01:28:23] 
[01:28:23] error[E0277]: can't compare `&realstd::vec::Vec<u8>` with `[{integer}; 12]`
[01:28:23]      |
[01:28:23]      |
[01:28:23] 1130 |         assert_eq!(writer.get_ref(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
[01:28:23]      |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `&realstd::vec::Vec<u8> == [{integer}; 12]`
[01:28:23]      |
[01:28:23]      = help: the trait `core::cmp::PartialEq<[{integer}; 12]>` is not implemented for `&realstd::vec::Vec<u8>`
[01:28:23] 
[01:28:23] 
[01:28:23] error[E0277]: can't compare `&realstd::vec::Vec<u8>` with `[{integer}; 12]`
[01:28:23]      |
[01:28:23]      |
[01:28:23] 1134 |         assert_eq!(writer.get_ref(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
[01:28:23]      |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `&realstd::vec::Vec<u8> == [{integer}; 12]`
[01:28:23]      |
[01:28:23]      = help: the trait `core::cmp::PartialEq<[{integer}; 12]>` is not implemented for `&realstd::vec::Vec<u8>`
[01:28:23] 
[01:28:30] error: aborting due to 9 previous errors
[01:28:30] 
[01:28:30] For more information about this error, try `rustc --explain E0277`.
[01:28:30] For more information about this error, try `rustc --explain E0277`.
[01:28:30] error: Could not compile `std`.
[01:28:30] 
[01:28:30] To learn more, run the command again with --verbose.
[01:28:30] 
[01:28:30] 
[01:28:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:28:30] 
[01:28:30] 
[01:28:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:28:30] Build completed unsuccessfully in 0:39:03
[01:28:30] Build completed unsuccessfully in 0:39:03
[01:28:30] make: *** [check] Error 1
[01:28:30] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00927a04
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
