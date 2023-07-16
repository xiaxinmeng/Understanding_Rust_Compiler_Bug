plain
W: GPG error: https://dl.hhvm.com/ubuntu trusty InRelease: The following signatures couldn't be verified because the public key is not available: NO_PUBKEY B4112585D386EB94
W: The repository 'https://dl.hhvm.com/ubuntu trusty InRelease' is not signed.
W: There is no public key available for the following key IDs:
---
Resolving deltas: 100% (611526/611526), completed with 4863 local objects.
---
[00:00:48] configure: rust.quiet-tests     := True
---
[00:37:53] ..........................................................................i.........................
[00:37:59] .................i..................................................................................
---
[00:38:33] ..........................................................................................i.........
[00:38:40] ..............................................................i.....................................
---
[00:39:30] .............................................i......................................................
---
[00:43:06] .............................i......................................................................
[00:43:20] ..............................................................i.....................................
[00:43:35] ...............................................i....................................................
[00:43:53] ....................................................................................................
[00:44:14] ....................................................................................................
[00:44:34] ....................................................................................................
[00:44:57] .i...............................................................................................i..
[00:45:26] ..........................................................................................test [run-pass] run-pass/mir_heavy_promoted.rs has been running for over 60 seconds
[00:45:31] ..........
[00:45:59] ....................................................................................................
[00:46:32] .............................................................ii.....................................
[00:47:18] ........................i....................................................i.ii...............test [run-pass] run-pass/saturating-float-casts.rs has been running for over 60 seconds
[00:47:20] ....
[00:47:57] .....................................................................................iiiiiii........
---
[00:49:53] ....................................i...............................................................
[00:50:01] ....................................................................................................
[00:50:08] ..................i............................................................ii.iii...............
[00:50:15] ....................................................................................................
[00:50:22] ........i..............................i............................................................
[00:50:29] ....................................................................................................
[00:50:36] ....................i...............................................................................
[00:50:43] ....................................................................................................
[00:50:53] ....................................................................................................
[00:51:03] ....................................................................................................
[00:51:13] ....................................................................................................
[00:51:26] ....................................................................................................
[00:51:34] .............i......................................................................................
[00:51:43] ................i..ii...............................................................................
[00:51:53] ....................................................................................................
[00:52:02] ....................................................................................................
[00:52:11] ..................................................................................i.................
[00:52:22] ............................i.......................................................................
---
[00:52:57] ............................i.......................................................................
[00:52:58] ....................................................................i...............................
[00:52:59] ................i.......................................................
---
[00:53:13] ...........i........................
---
[00:53:41] i...i..ii....i.............ii........iii......i..i...i...ii..i..i..ii.....
---
[00:53:43] i.......i......................i......
---
[00:54:19] iiii.......i..i........i..i.i.............i..........iiii...........i...i..........ii.i.i.......ii..
[00:54:20] ....ii...
---
[01:02:45] ...i................................................................................................
---
[01:04:32] .....................................i..............................................................
[01:04:50] ....................................................................................................
[01:05:08] ..........................................i.........................................................
---
[01:06:28] .........................................................ii.........................................
---
[01:06:54] FFFFFFF.............................................................................................
[01:07:10] ....................................................................................................
[01:07:28] ..............................................................i.....................................
---
[01:10:18] ---- ../stdsimd/coresimd/macros.rs - coresimd::x86::__m128 (line 27) stdout ----
[01:10:18]  error: this feature has been stable since 1.27.0. Attribute no longer needed
[01:10:18]  --> ../stdsimd/coresimd/macros.rs:28:12
[01:10:18]   |
[01:10:18] 3 | #![feature(cfg_target_feature, target_feature, stdsimd)]
---
[01:10:18]   = note: #[deny(stable_features)] implied by #[deny(warnings)]
[01:10:18]
[01:10:18] error: this feature has been stable since 1.27.0. Attribute no longer needed
[01:10:18]  --> ../stdsimd/coresimd/macros.rs:28:32
[01:10:18]   |
[01:10:18] 3 | #![feature(cfg_target_feature, target_feature, stdsimd)]
[01:10:18]   |                                ^^^^^^^^^^^^^^
[01:10:18]
[01:10:18] thread '../stdsimd/coresimd/macros.rs - coresimd::x86::__m128 (line 27)' panicked at 'couldn't compile the test', librustdoc/test.rs:306:13
[01:10:18] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:10:18]
[01:10:18] ---- ../stdsimd/coresimd/macros.rs - coresimd::x86::__m128d (line 27) stdout ----
[01:10:18]  error: this feature has been stable since 1.27.0. Attribute no longer needed
[01:10:18]  --> ../stdsimd/coresimd/macros.rs:28:12
[01:10:18]   |
[01:10:18] 3 | #![feature(cfg_target_feature, target_feature, stdsimd)]
---
[01:10:18]   = note: #[deny(stable_features)] implied by #[deny(warnings)]
[01:10:18]
[01:10:18] error: this feature has been stable since 1.27.0. Attribute no longer needed
[01:10:18]  --> ../stdsimd/coresimd/macros.rs:28:32
[01:10:18]   |
[01:10:18] 3 | #![feature(cfg_target_feature, target_feature, stdsimd)]
[01:10:18]   |                                ^^^^^^^^^^^^^^
[01:10:18]
[01:10:18] thread '../stdsimd/coresimd/macros.rs - coresimd::x86::__m128d (line 27)' panicked at 'couldn't compile the test', librustdoc/test.rs:306:13
[01:10:18]
[01:10:18] ---- ../stdsimd/coresimd/macros.rs - coresimd::x86::__m128i (line 34) stdout ----
[01:10:18]  error: this feature has been stable since 1.27.0. Attribute no longer needed
[01:10:18]  --> ../stdsimd/coresimd/macros.rs:35:12
[01:10:18]   |
[01:10:18] 3 | #![feature(cfg_target_feature, target_feature, stdsimd)]
---
[01:10:18]   = note: #[deny(stable_features)] implied by #[deny(warnings)]
[01:10:18]
[01:10:18] error: this feature has been stable since 1.27.0. Attribute no longer needed
[01:10:18]  --> ../stdsimd/coresimd/macros.rs:35:32
[01:10:18]   |
[01:10:18] 3 | #![feature(cfg_target_feature, target_feature, stdsimd)]
[01:10:18]   |                                ^^^^^^^^^^^^^^
[01:10:18]
[01:10:18] thread '../stdsimd/coresimd/macros.rs - coresimd::x86::__m128i (line 34)' panicked at 'couldn't compile the test', librustdoc/test.rs:306:13
[01:10:18]
[01:10:18] ---- ../stdsimd/coresimd/macros.rs - coresimd::x86::__m256 (line 27) stdout ----
[01:10:18]  error: this feature has been stable since 1.27.0. Attribute no longer needed
[01:10:18]  --> ../stdsimd/coresimd/macros.rs:28:12
[01:10:18]   |
[01:10:18] 3 | #![feature(cfg_target_feature, target_feature, stdsimd)]
---
[01:10:18]   = note: #[deny(stable_features)] implied by #[deny(warnings)]
[01:10:18]
[01:10:18] error: this feature has been stable since 1.27.0. Attribute no longer needed
[01:10:18]  --> ../stdsimd/coresimd/macros.rs:28:32
[01:10:18]   |
[01:10:18] 3 | #![feature(cfg_target_feature, target_feature, stdsimd)]
[01:10:18]   |                                ^^^^^^^^^^^^^^
[01:10:18]
[01:10:18] thread '../stdsimd/coresimd/macros.rs - coresimd::x86::__m256 (line 27)' panicked at 'couldn't compile the test', librustdoc/test.rs:306:13
[01:10:18]
[01:10:18] ---- ../stdsimd/coresimd/macros.rs - coresimd::x86::__m64 (line 34) stdout ----
[01:10:18]  error: this feature has been stable since 1.27.0. Attribute no longer needed
[01:10:18]  --> ../stdsimd/coresimd/macros.rs:35:12
[01:10:18]   |
[01:10:18] 3 | #![feature(cfg_target_feature, target_feature, stdsimd)]
---
[01:10:18]   = note: #[deny(stable_features)] implied by #[deny(warnings)]
[01:10:18]
[01:10:18] error: this feature has been stable since 1.27.0. Attribute no longer needed
[01:10:18]  --> ../stdsimd/coresimd/macros.rs:35:32
[01:10:18]   |
[01:10:18] 3 | #![feature(cfg_target_feature, target_feature, stdsimd)]
[01:10:18]   |                                ^^^^^^^^^^^^^^
[01:10:18]
[01:10:18] thread '../stdsimd/coresimd/macros.rs - coresimd::x86::__m64 (line 34)' panicked at 'couldn't compile the test', librustdoc/test.rs:306:13
[01:10:18]
[01:10:18] ---- ../stdsimd/coresimd/macros.rs - coresimd::x86::__m256d (line 27) stdout ----
[01:10:18]  error: this feature has been stable since 1.27.0. Attribute no longer needed
[01:10:18]  --> ../stdsimd/coresimd/macros.rs:28:12
[01:10:18]   |
[01:10:18] 3 | #![feature(cfg_target_feature, target_feature, stdsimd)]
---
[01:10:18]   = note: #[deny(stable_features)] implied by #[deny(warnings)]
[01:10:18]
[01:10:18] error: this feature has been stable since 1.27.0. Attribute no longer needed
[01:10:18]  --> ../stdsimd/coresimd/macros.rs:28:32
[01:10:18]   |
[01:10:18] 3 | #![feature(cfg_target_feature, target_feature, stdsimd)]
[01:10:18]   |                                ^^^^^^^^^^^^^^
[01:10:18]
[01:10:18] thread '../stdsimd/coresimd/macros.rs - coresimd::x86::__m256d (line 27)' panicked at 'couldn't compile the test', librustdoc/test.rs:306:13
[01:10:18]
[01:10:18] ---- ../stdsimd/coresimd/macros.rs - coresimd::x86::__m256i (line 31) stdout ----
[01:10:18]  error: this feature has been stable since 1.27.0. Attribute no longer needed
[01:10:18]  --> ../stdsimd/coresimd/macros.rs:32:12
[01:10:18]   |
[01:10:18] 3 | #![feature(cfg_target_feature, target_feature, stdsimd)]
---
[01:10:18]   = note: #[deny(stable_features)] implied by #[deny(warnings)]
[01:10:18]
[01:10:18] error: this feature has been stable since 1.27.0. Attribute no longer needed
[01:10:18]  --> ../stdsimd/coresimd/macros.rs:32:32
[01:10:18]   |
[01:10:18] 3 | #![feature(cfg_target_feature, target_feature, stdsimd)]
[01:10:18]   |                                ^^^^^^^^^^^^^^
[01:10:18]
[01:10:18] thread '../stdsimd/coresimd/macros.rs - coresimd::x86::__m256i (line 31)' panicked at 'couldn't compile the test', librustdoc/test.rs:306:13
---
[01:10:18] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:10:18] expected success, got: exit code: 101
[01:10:18]
[01:10:18]
[01:10:18] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:10:18] Build completed unsuccessfully in 0:33:32
[01:10:18] Makefile:58: recipe for target 'check' failed
[01:10:18] make: *** [check] Error 1
---
$ dmesg | grep -i kill
[   10.853990] init: failsafe main process (1093) killed by TERM signal
