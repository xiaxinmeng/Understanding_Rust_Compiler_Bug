plain
travis_time:end:33e8a6a5:start=1542354121775092564,finish=1542354182173663961,duration=60398571397
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:04:01]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:04:01]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:04:02]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:04:02]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:04:10] warning: unnecessary `unsafe` block
[00:04:10]     |
[00:04:10]     |
[00:04:10] 73  |                       $Ty(unsafe { NonZero(n) })
[00:04:10]     |                           ^^^^^^ unnecessary `unsafe` block
[00:04:10] ...
[00:04:10] 110 | / nonzero_integers! {
[00:04:10] 111 | |     NonZeroU8(u8);
[00:04:10] 112 | |     NonZeroU16(u16);
[00:04:10] 113 | |     NonZeroU32(u32);
[00:04:10] 116 | |     NonZeroUsize(usize);
[00:04:10] 117 | | }
[00:04:10]     | |_- in this macro invocation
[00:04:10]     |
[00:04:10]     |
[00:04:10]     = note: #[warn(unused_unsafe)] on by default
[00:04:10] 
[00:04:10] warning: unnecessary `unsafe` block
[00:04:10]     |
[00:04:10]     |
[00:04:10] 73  |                       $Ty(unsafe { NonZero(n) })
[00:04:10]     |                           ^^^^^^ unnecessary `unsafe` block
[00:04:10] ...
[00:04:10] 110 | / nonzero_integers! {
[00:04:10] 111 | |     NonZeroU8(u8);
[00:04:10] 112 | |     NonZeroU16(u16);
[00:04:10] 113 | |     NonZeroU32(u32);
[00:04:10] 116 | |     NonZeroUsize(usize);
[00:04:10] 117 | | }
[00:04:10]     | |_- in this macro invocation
[00:04:10] 
[00:04:10] 
[00:04:10] warning: unnecessary `unsafe` block
[00:04:10]     |
[00:04:10]     |
[00:04:10] 81  |                           Some($Ty(unsafe { NonZero(n) }))
[00:04:10]     |                                    ^^^^^^ unnecessary `unsafe` block
[00:04:10] ...
[00:04:10] 110 | / nonzero_integers! {
[00:04:10] 111 | |     NonZeroU8(u8);
[00:04:10] 112 | |     NonZeroU16(u16);
[00:04:10] 113 | |     NonZeroU32(u32);
[00:04:10] 116 | |     NonZeroUsize(usize);
[00:04:10] 117 | | }
[00:04:10]     | |_- in this macro invocation
[00:04:10] 
[00:04:10] 
[00:04:10] warning: unnecessary `unsafe` block
[00:04:10]     --> libcore/ptr.rs:2755:36
[00:04:10]      |
[00:04:10] 2755 |             Some(Unique { pointer: unsafe { NonZero(ptr as _) }, _marker: PhantomData })
[00:04:10]      |                                    ^^^^^^ unnecessary `unsafe` block
[00:04:10] 
[00:04:10] warning: unnecessary `unsafe` block
[00:04:10]     --> libcore/ptr.rs:2811:27
[00:04:10]      |
[00:04:10] 2811 |         Unique { pointer: unsafe { NonZero(reference as _) }, _marker: PhantomData }
[00:04:10]      |                           ^^^^^^ unnecessary `unsafe` block
[00:04:10] 
[00:04:10] warning: unnecessary `unsafe` block
[00:04:10]     --> libcore/ptr.rs:2818:27
[00:04:10]      |
[00:04:10] 2818 |         Unique { pointer: unsafe { NonZero(reference as _) }, _marker: PhantomData }
[00:04:10]      |                           ^^^^^^ unnecessary `unsafe` block
[00:04:10] 
[00:04:10] warning: unnecessary `unsafe` block
[00:04:10]     --> libcore/ptr.rs:2891:28
[00:04:10]      |
[00:04:10] 2891 |         NonNull { pointer: unsafe { NonZero(ptr as _) } }
[00:04:10]      |                            ^^^^^^ unnecessary `unsafe` block
[00:04:10] 
[00:04:10] warning: unnecessary `unsafe` block
[00:04:10]     --> libcore/ptr.rs:2899:37
[00:04:10]      |
[00:04:10] 2899 |             Some(NonNull { pointer: unsafe { NonZero(ptr as _) } })
[00:04:10]      |                                     ^^^^^^ unnecessary `unsafe` block
[00:04:10] 
[00:04:10] warning: unnecessary `unsafe` block
[00:04:10]     --> libcore/ptr.rs:3021:28
[00:04:10]      |
[00:04:10] 3021 |         NonNull { pointer: unsafe { NonZero(reference as _) } }
[00:04:10]      |                            ^^^^^^ unnecessary `unsafe` block
[00:04:10] 
[00:04:10] warning: unnecessary `unsafe` block
[00:04:10]     --> libcore/ptr.rs:3029:28
[00:04:10]      |
[00:04:10] 3029 |         NonNull { pointer: unsafe { NonZero(reference as _) } }
[00:04:10]      |                            ^^^^^^ unnecessary `unsafe` block
[00:04:10] 
[00:04:10] warning: unnecessary `unsafe` block
[00:04:10]    |
[00:04:10]    |
[00:04:10] 27 |         unsafe { NonZero(self.0) }
[00:04:10]    |         ^^^^^^ unnecessary `unsafe` block
[00:04:17]    Compiling libc v0.0.0 (/checkout/src/rustc/libc_shim)
[00:04:17]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[00:04:18]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:04:22]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
---
[00:07:12]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:07:18] error[E0405]: cannot find trait `Decodable` in this scope
[00:07:18]    --> librustc/hir/mod.rs:133:5
[00:07:18]     |
[00:07:18] 133 | /     newtype_index! {
[00:07:18] 134 | |         pub struct ItemLocalId { .. }
[00:07:18]     | |_____^ not found in this scope
[00:07:18]     |
[00:07:18]     = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:07:18] help: possible candidate is found in another module, you can import it into scope
[00:07:18] help: possible candidate is found in another module, you can import it into scope
[00:07:18]     |
[00:07:18] 124 |     use rustc_serialize::Decodable;
[00:07:18]     |
[00:07:18] 
[00:07:18] error[E0405]: cannot find trait `Decoder` in this scope
[00:07:18]    --> librustc/hir/mod.rs:133:5
[00:07:18] 133 | /     newtype_index! {
[00:07:18] 133 | /     newtype_index! {
[00:07:18] 134 | |         pub struct ItemLocalId { .. }
[00:07:18]     | |_____^ not found in this scope
[00:07:18]     |
[00:07:18]     = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:07:18] help: possible candidate is found in another module, you can import it into scope
