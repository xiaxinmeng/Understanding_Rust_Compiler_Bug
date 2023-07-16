plain
travis_time:end:21cdb6a0:start=1542981590171658239,finish=1542981592548440156,duration=2376781917
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:02:47]   Downloaded winapi-util v0.1.1
[00:02:47]   Downloaded rustc-ap-rustc_cratesio_shim v297.0.0
[00:02:47]   Downloaded smallvec v0.6.5
[00:02:47]   Downloaded rustc-demangle v0.1.9
[00:02:47]   Downloaded rand_xorshift v0.1.0
[00:02:47]   Downloaded unicode-bidi v0.3.4
[00:02:47]   Downloaded regex-syntax v0.5.6
[00:02:47]   Downloaded bitflags v0.9.1
[00:02:47]   Downloaded rayon-core v1.4.0
---
[00:04:10]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:04:10]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:04:11]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:04:11]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:04:26]    Compiling rustc-std-workspace-core v1.0.0 (/checkout/src/tools/rustc-std-workspace-core)
[00:04:28]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:04:33]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:04:52]     Finished release [optimized] target(s) in 53.81s
[00:04:52] Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
[00:21:22]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:21:22]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:21:23]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:21:23]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:21:58]    Compiling rustc-std-workspace-core v1.0.0 (/checkout/src/tools/rustc-std-workspace-core)
[00:22:00]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:22:08]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:22:31]     Finished release [optimized] target(s) in 1m 20s
[00:22:31] Copying stage1 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---

[00:47:32] Documenting book redirect pages (x86_64-unknown-linux-gnu)
[00:47:33] Documenting stage2 std (x86_64-unknown-linux-gnu)
[00:47:33]     Checking core v0.0.0 (/checkout/src/libcore)
[00:48:10]     Checking rustc-std-workspace-core v1.0.0 (/checkout/src/tools/rustc-std-workspace-core)
[00:48:12]  Documenting alloc v0.0.0 (/checkout/src/liballoc)
[00:48:17]     Finished release [optimized] target(s) in 44.02s
[00:48:17]  Documenting core v0.0.0 (/checkout/src/libcore)
[00:48:37] warning: `[chunks]` cannot be resolved, ignoring it...
---
[00:53:50] .................................................................................................... 100/5049
[00:53:53] .................................................................................................... 200/5049
[00:53:56] .............................ii............................................ii...................ii.. 300/5049
[00:53:59] ..............................................................................................iii... 400/5049
[00:54:02] .....iiiiiiii.iii............................iii...........................................i........ 500/5049
[00:54:09] .................................................................................................... 700/5049
[00:54:15] ...................................................................................i...........i.... 800/5049
[00:54:18] .................................................................................................... 900/5049
[00:54:18] .................................................................................................... 900/5049
[00:54:22] ..iiiii...................iiiiii.................................................................... 1000/5049
[00:54:24] .............................................................................iiiiiiii............... 1100/5049
[00:54:28] .................................................................................................... 1300/5049
[00:54:31] .................................................................................................... 1400/5049
[00:54:33] .................................i.................................................................. 1500/5049
[00:54:36] ..i.........ii.........................................................i............................ 1600/5049
---
[00:54:58] .................................................................................................... 2200/5049
[00:55:02] .................................................................................................... 2300/5049
[00:55:05] .................................................................................................... 2400/5049
[00:55:10] .................................................................................................... 2500/5049
[00:55:13] ...........................................................................................iiiiiiiii 2600/5049
[00:55:20] .........................................................ii......................................... 2800/5049
[00:55:23] .................................................................................................... 2900/5049
[00:55:27] .................................................................................................... 3000/5049
[00:55:30] .......................................................i............................................ 3100/5049
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:10:03] 
[01:10:03] running 117 tests
[01:10:06] i..ii...iii..iii.i....i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..i 100/117
[01:10:06] i.i.....iiii.....
[01:10:06] 
[01:10:06]  finished in 3.574
[01:10:06] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:10:21] 
[01:10:21] running 118 tests
[01:10:43] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:10:47] ......iii.i.....ii
[01:10:47] 
[01:10:47]  finished in 26.906
[01:10:47] travis_fold:end:test_debuginfo

---
Testing alloc stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:21:33]    Compiling semver-parser v0.7.0
[01:21:33]    Compiling rand_core v0.3.0
[01:21:33]    Compiling rand_core v0.2.1
[01:21:34]    Compiling rand_hc v0.1.0
[01:21:34]    Compiling rand_xorshift v0.1.0
[01:21:34]    Compiling rand_isaac v0.1.0
[01:21:34] error[E0407]: method `seed_from_u64` is not a member of trait `SeedableRng`
[01:21:34]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand_isaac-0.1.0/src/isaac.rs:126:5
[01:21:34]     |
[01:21:34] 126 | /     fn seed_from_u64(seed: u64) -> Self {
[01:21:34] 127 | |         IsaacRng(BlockRng::<IsaacCore>::seed_from_u64(seed))
[01:21:34] 128 | |     }
[01:21:34]     | |_____^ not a member of trait `SeedableRng`
[01:21:34] 
[01:21:34] error[E0407]: method `seed_from_u64` is not a member of trait `SeedableRng`
[01:21:34]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand_isaac-0.1.0/src/isaac.rs:331:5
[01:21:34]     |
[01:21:34] 331 | /     fn seed_from_u64(seed: u64) -> Self {
[01:21:34] 332 | |         let mut key = [w(0); RAND_SIZE];
[01:21:34] 333 | |         key[0] = w(seed as u32);
[01:21:34] 334 | |         key[1] = w((seed >> 32) as u32);
[01:21:34] ...   |
[01:21:34] 341 | |         Self::init(key, 1)
[01:21:34] 342 | |     }
[01:21:34]     | |_____^ not a member of trait `SeedableRng`
[01:21:34] 
[01:21:34] error[E0407]: method `seed_from_u64` is not a member of trait `SeedableRng`
[01:21:34]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand_isaac-0.1.0/src/isaac64.rs:116:5
[01:21:34]     |
[01:21:34] 116 | /     fn seed_from_u64(seed: u64) -> Self {
[01:21:34] 117 | |         Isaac64Rng(BlockRng64::<Isaac64Core>::seed_from_u64(seed))
[01:21:34] 118 | |     }
[01:21:34]     | |_____^ not a member of trait `SeedableRng`
[01:21:34] 
[01:21:34] error[E0407]: method `seed_from_u64` is not a member of trait `SeedableRng`
[01:21:34]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand_isaac-0.1.0/src/isaac64.rs:301:5
[01:21:34]     |
[01:21:34] 301 | /     fn seed_from_u64(seed: u64) -> Self {
[01:21:34] 302 | |         let mut key = [w(0); RAND_SIZE];
[01:21:34] 303 | |         key[0] = w(seed);
[01:21:34] 304 | |         // Initialize with only one pass.
[01:21:34] ...   |
[01:21:34] 310 | |         Self::init(key, 1)
[01:21:34] 311 | |     }
[01:21:34]     | |_____^ not a member of trait `SeedableRng`
[01:21:34] 
[01:21:34] error[E0599]: no function or associated item named `seed_from_u64` found for type `rand_core::block::BlockRng<isaac::IsaacCore>` in the current scope
[01:21:34]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand_isaac-0.1.0/src/isaac.rs:127:18
[01:21:34]     |
[01:21:34] 127 |         IsaacRng(BlockRng::<IsaacCore>::seed_from_u64(seed))
[01:21:34]     |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ function or associated item not found in `rand_core::block::BlockRng<isaac::IsaacCore>`
[01:21:34] 
[01:21:34] error[E0599]: no function or associated item named `seed_from_u64` found for type `isaac::IsaacRng` in the current scope
[01:21:34]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand_isaac-0.1.0/src/isaac.rs:141:9
[01:21:34]     |
[01:21:34] 94  | pub struct IsaacRng(BlockRng<IsaacCore>);
[01:21:34]     | ----------------------------------------- function or associated item `seed_from_u64` not found for this
[01:21:34] ...
[01:21:34] 141 |         Self::seed_from_u64(seed)
[01:21:34]     |         ^^^^^^^^^^^^^^^^^^^ function or associated item not found in `isaac::IsaacRng`
[01:21:34]     |
[01:21:34]     = help: did you mean `new_from_u64`?
[01:21:34] 
[01:21:34] error[E0599]: no function or associated item named `seed_from_u64` found for type `rand_core::block::BlockRng64<isaac64::Isaac64Core>` in the current scope
[01:21:34]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand_isaac-0.1.0/src/isaac64.rs:117:20
[01:21:34]     |
[01:21:34] 117 |         Isaac64Rng(BlockRng64::<Isaac64Core>::seed_from_u64(seed))
[01:21:34]     |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ function or associated item not found in `rand_core::block::BlockRng64<isaac64::Isaac64Core>`
[01:21:34] 
[01:21:34] error[E0599]: no function or associated item named `seed_from_u64` found for type `isaac64::Isaac64Rng` in the current scope
[01:21:34]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand_isaac-0.1.0/src/isaac64.rs:131:9
[01:21:34]     |
[01:21:34] 84  | pub struct Isaac64Rng(BlockRng64<Isaac64Core>);
[01:21:34]     | ----------------------------------------------- function or associated item `seed_from_u64` not found for this
[01:21:34] ...
[01:21:34] 131 |         Self::seed_from_u64(seed)
[01:21:34]     |         ^^^^^^^^^^^^^^^^^^^ function or associated item not found in `isaac64::Isaac64Rng`
[01:21:34]     |
[01:21:34]     = help: did you mean `new_from_u64`?
[01:21:34] 
[01:21:34] error[E0599]: no function or associated item named `seed_from_u64` found for type `isaac64::Isaac64Core` in the current scope
[01:21:34]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand_isaac-0.1.0/src/isaac64.rs:283:9
[01:21:34]     |
[01:21:34] 138 | pub struct Isaac64Core {
[01:21:34]     | ---------------------- function or associated item `seed_from_u64` not found for this
[01:21:34] ...
[01:21:34] 283 |         Self::seed_from_u64(seed)
[01:21:34]     |         ^^^^^^^^^^^^^^^^^^^ function or associated item not found
