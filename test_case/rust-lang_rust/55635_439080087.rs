plain
travis_time:end:14fdee2c:start=1542295414436351932,finish=1542295416889508839,duration=2453156907
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:03:58]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:03:58]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:03:59]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:03:59]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:04:07] warning: unnecessary `unsafe` block
[00:04:07]     |
[00:04:07]     |
[00:04:07] 73  |                       $Ty(unsafe { NonZero(n) })
[00:04:07]     |                           ^^^^^^ unnecessary `unsafe` block
[00:04:07] ...
[00:04:07] 110 | / nonzero_integers! {
[00:04:07] 111 | |     NonZeroU8(u8);
[00:04:07] 112 | |     NonZeroU16(u16);
[00:04:07] 113 | |     NonZeroU32(u32);
[00:04:07] 116 | |     NonZeroUsize(usize);
[00:04:07] 117 | | }
[00:04:07]     | |_- in this macro invocation
[00:04:07]     |
[00:04:07]     |
[00:04:07]     = note: #[warn(unused_unsafe)] on by default
[00:04:07] 
[00:04:07] warning: unnecessary `unsafe` block
[00:04:07]     |
[00:04:07]     |
[00:04:07] 73  |                       $Ty(unsafe { NonZero(n) })
[00:04:07]     |                           ^^^^^^ unnecessary `unsafe` block
[00:04:07] ...
[00:04:07] 110 | / nonzero_integers! {
[00:04:07] 111 | |     NonZeroU8(u8);
[00:04:07] 112 | |     NonZeroU16(u16);
[00:04:07] 113 | |     NonZeroU32(u32);
[00:04:07] 116 | |     NonZeroUsize(usize);
[00:04:07] 117 | | }
[00:04:07]     | |_- in this macro invocation
[00:04:07] 
[00:04:07] 
[00:04:07] warning: unnecessary `unsafe` block
[00:04:07]     |
[00:04:07]     |
[00:04:07] 81  |                           Some($Ty(unsafe { NonZero(n) }))
[00:04:07]     |                                    ^^^^^^ unnecessary `unsafe` block
[00:04:07] ...
[00:04:07] 110 | / nonzero_integers! {
[00:04:07] 111 | |     NonZeroU8(u8);
[00:04:07] 112 | |     NonZeroU16(u16);
[00:04:07] 113 | |     NonZeroU32(u32);
[00:04:07] 116 | |     NonZeroUsize(usize);
[00:04:07] 117 | | }
[00:04:07]     | |_- in this macro invocation
[00:04:07] 
[00:04:07] 
[00:04:07] warning: unnecessary `unsafe` block
[00:04:07]     --> libcore/ptr.rs:2755:36
[00:04:07]      |
[00:04:07] 2755 |             Some(Unique { pointer: unsafe { NonZero(ptr as _) }, _marker: PhantomData })
[00:04:07]      |                                    ^^^^^^ unnecessary `unsafe` block
[00:04:07] 
[00:04:07] warning: unnecessary `unsafe` block
[00:04:07]     --> libcore/ptr.rs:2811:27
[00:04:07]      |
[00:04:07] 2811 |         Unique { pointer: unsafe { NonZero(reference as _) }, _marker: PhantomData }
[00:04:07]      |                           ^^^^^^ unnecessary `unsafe` block
[00:04:07] 
[00:04:07] warning: unnecessary `unsafe` block
[00:04:07]     --> libcore/ptr.rs:2818:27
[00:04:07]      |
[00:04:07] 2818 |         Unique { pointer: unsafe { NonZero(reference as _) }, _marker: PhantomData }
[00:04:07]      |                           ^^^^^^ unnecessary `unsafe` block
[00:04:07] 
[00:04:07] warning: unnecessary `unsafe` block
[00:04:07]     --> libcore/ptr.rs:2891:28
[00:04:07]      |
[00:04:07] 2891 |         NonNull { pointer: unsafe { NonZero(ptr as _) } }
[00:04:07]      |                            ^^^^^^ unnecessary `unsafe` block
[00:04:07] 
[00:04:07] warning: unnecessary `unsafe` block
[00:04:07]     --> libcore/ptr.rs:2899:37
[00:04:07]      |
[00:04:07] 2899 |             Some(NonNull { pointer: unsafe { NonZero(ptr as _) } })
[00:04:07]      |                                     ^^^^^^ unnecessary `unsafe` block
[00:04:07] 
[00:04:07] warning: unnecessary `unsafe` block
[00:04:07]     --> libcore/ptr.rs:3021:28
[00:04:07]      |
[00:04:07] 3021 |         NonNull { pointer: unsafe { NonZero(reference as _) } }
[00:04:07]      |                            ^^^^^^ unnecessary `unsafe` block
[00:04:07] 
[00:04:07] warning: unnecessary `unsafe` block
[00:04:07]     --> libcore/ptr.rs:3029:28
[00:04:07]      |
[00:04:07] 3029 |         NonNull { pointer: unsafe { NonZero(reference as _) } }
[00:04:07]      |                            ^^^^^^ unnecessary `unsafe` block
[00:04:07] 
[00:04:07] warning: unnecessary `unsafe` block
[00:04:07]    |
[00:04:07]    |
[00:04:07] 27 |         unsafe { NonZero(self.0) }
[00:04:07]    |         ^^^^^^ unnecessary `unsafe` block
[00:04:14]    Compiling libc v0.0.0 (/checkout/src/rustc/libc_shim)
[00:04:14]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[00:04:15]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:04:19]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
---
[00:05:28]    Compiling arena v0.0.0 (/checkout/src/libarena)
[00:05:29] error[E0405]: cannot find trait `Decodable` in this scope
[00:05:29]    --> librustc_target/abi/mod.rs:830:1
[00:05:29]     |
[00:05:29] 830 | / newtype_index! {
[00:05:29] 831 | |     pub struct VariantIdx { .. }
[00:05:29]     | |_^ not found in this scope
[00:05:29]     |
[00:05:29]     = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:05:29] help: possible candidate is found in another module, you can import it into scope
[00:05:29] help: possible candidate is found in another module, you can import it into scope
[00:05:29]     |
[00:05:29] 11  | use rustc_serialize::Decodable;
[00:05:29]     |
[00:05:29] 
[00:05:29] error[E0405]: cannot find trait `Decoder` in this scope
[00:05:29]    --> librustc_target/abi/mod.rs:830:1
[00:05:29]     |
[00:05:29] 830 | / newtype_index! {
[00:05:29] 831 | |     pub struct VariantIdx { .. }
[00:05:29]     | |_^ not found in this scope
[00:05:29]     |
[00:05:29]     = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:05:29] help: possible candidate is found in another module, you can import it into scope
---
[00:05:29] warning: build failed, waiting for other jobs to finish...
[00:05:32] error: build failed
[00:05:32] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:05:32] expected success, got: exit code: 101
[00:05:32] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1101:9
[00:05:32] travis_fold:end:stage0-rustc

[00:05:32] travis_time:end:stage0-rustc:start=1542295717098970389,finish=1542295760547264244,duration=43448293855


[00:05:32] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:05:32] Build completed unsuccessfully in 0:01:45
[00:05:32] Makefile:28: recipe for target 'all' failed
[00:05:32] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0c00c058
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Nov 15 15:29:20 UTC 2018
