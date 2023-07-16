plain
[00:04:44]    Compiling aho-corasick v0.7.3
[00:04:45]    Compiling bstr v0.1.4
[00:04:45]    Compiling clap v2.33.0
[00:04:45]    Compiling tar v0.4.26
[00:04:49] error[E0277]: the trait bound `rand_hc::Hc128Rng: rand_core::SeedableRng` is not satisfied
[00:04:49]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/std.rs:55:6
[00:04:49]    |
[00:04:49] 55 | impl SeedableRng for StdRng {
[00:04:49]    |      ^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_hc::Hc128Rng`
[00:04:49] 
[00:04:49] error[E0277]: the trait bound `rand_isaac::IsaacRng: rand_core::SeedableRng` is not satisfied
[00:04:49]    |
[00:04:49]    |
[00:04:49] 50 | impl SeedableRng for IsaacRng {
[00:04:49]    |      ^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_isaac::IsaacRng`
[00:04:49] 
[00:04:49] error[E0277]: the trait bound `rand_isaac::Isaac64Rng: rand_core::SeedableRng` is not satisfied
[00:04:49]    |
[00:04:49]    |
[00:04:49] 96 | impl SeedableRng for Isaac64Rng {
[00:04:49]    |      ^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_isaac::Isaac64Rng`
[00:04:49] 
[00:04:49] error[E0277]: the trait bound `rand_hc::Hc128Rng: rand_core::SeedableRng` is not satisfied
[00:04:49]     |
[00:04:49]     |
[00:04:49] 198 | impl SeedableRng for Hc128Rng {
[00:04:49]     |      ^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_hc::Hc128Rng`
[00:04:49] 
[00:04:49] error[E0277]: the trait bound `rand_xorshift::XorShiftRng: rand_core::SeedableRng` is not satisfied
[00:04:49]     |
[00:04:49]     |
[00:04:49] 239 | impl SeedableRng for XorShiftRng {
[00:04:49]     |      ^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_xorshift::XorShiftRng`
[00:04:49] 
[00:04:49] error[E0277]: the trait bound `rand_hc::Hc128Rng: rand_core::SeedableRng` is not satisfied
[00:04:49]     |
[00:04:49]     |
[00:04:49] 279 | impl SeedableRng for StdRng {
[00:04:49]     |      ^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_hc::Hc128Rng`
[00:04:49] 
[00:04:49] error[E0277]: the trait bound `rand_hc::Hc128Core: rand_core::block::BlockRngCore` is not satisfied
[00:04:49]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/thread.rs:73:5
[00:04:49]    |
[00:04:49] 73 |     rng: *mut ReseedingRng<Hc128Core, EntropyRng>,
[00:04:49]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rand_core::block::BlockRngCore` is not implemented for `rand_hc::Hc128Core`
[00:04:49]    |
[00:04:49] note: required by `rngs::adapter::reseeding::ReseedingRng`
[00:04:49]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/adapter/reseeding.rs:88:1
[00:04:49]    |
[00:04:49] 88 | / pub struct ReseedingRng<R, Rsdr>(BlockRng<ReseedingCore<R, Rsdr>>)
[00:04:49] 89 | | where R: BlockRngCore + SeedableRng,
[00:04:49] 90 | |       Rsdr: RngCore;
[00:04:49] 
[00:04:49] 
[00:04:49] error[E0277]: the trait bound `rand_hc::Hc128Core: rand_core::SeedableRng` is not satisfied
[00:04:49]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/thread.rs:73:5
[00:04:49]    |
[00:04:49] 73 |     rng: *mut ReseedingRng<Hc128Core, EntropyRng>,
[00:04:49]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_hc::Hc128Core`
[00:04:49]    |
[00:04:49] note: required by `rngs::adapter::reseeding::ReseedingRng`
[00:04:49]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/adapter/reseeding.rs:88:1
[00:04:49]    |
[00:04:49] 88 | / pub struct ReseedingRng<R, Rsdr>(BlockRng<ReseedingCore<R, Rsdr>>)
[00:04:49] 89 | | where R: BlockRngCore + SeedableRng,
[00:04:49] 90 | |       Rsdr: RngCore;
[00:04:49] 
[00:04:49] 
[00:04:49] error[E0277]: the trait bound `rand_hc::Hc128Core: rand_core::block::BlockRngCore` is not satisfied
[00:04:49]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/thread.rs:76:1
[00:04:49] 76 | / thread_local!(
[00:04:49] 76 | / thread_local!(
[00:04:49] 77 | |     static THREAD_RNG_KEY: UnsafeCell<ReseedingRng<Hc128Core, EntropyRng>> = {
[00:04:49] 78 | |         let mut entropy_source = EntropyRng::new();
[00:04:49] 79 | |         let r = Hc128Core::from_rng(&mut entropy_source).unwrap_or_else(|err|
[00:04:49] 85 | |     }
[00:04:49] 86 | | );
[00:04:49] 86 | | );
[00:04:49]    | |__^ the trait `rand_core::block::BlockRngCore` is not implemented for `rand_hc::Hc128Core`
[00:04:49]    |
[00:04:49] note: required by `rngs::adapter::reseeding::ReseedingRng`
[00:04:49]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/adapter/reseeding.rs:88:1
[00:04:49]    |
[00:04:49] 88 | / pub struct ReseedingRng<R, Rsdr>(BlockRng<ReseedingCore<R, Rsdr>>)
[00:04:49] 89 | | where R: BlockRngCore + SeedableRng,
[00:04:49] 90 | |       Rsdr: RngCore;
[00:04:49]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:04:49] 
[00:04:49] 
[00:04:49] error[E0277]: the trait bound `rand_hc::Hc128Core: rand_core::SeedableRng` is not satisfied
[00:04:49]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/thread.rs:76:1
[00:04:49] 76 | / thread_local!(
[00:04:49] 76 | / thread_local!(
[00:04:49] 77 | |     static THREAD_RNG_KEY: UnsafeCell<ReseedingRng<Hc128Core, EntropyRng>> = {
[00:04:49] 78 | |         let mut entropy_source = EntropyRng::new();
[00:04:49] 79 | |         let r = Hc128Core::from_rng(&mut entropy_source).unwrap_or_else(|err|
[00:04:49] 85 | |     }
[00:04:49] 86 | | );
[00:04:49] 86 | | );
[00:04:49]    | |__^ the trait `rand_core::SeedableRng` is not implemented for `rand_hc::Hc128Core`
[00:04:49]    |
[00:04:49] note: required by `rngs::adapter::reseeding::ReseedingRng`
[00:04:49]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/adapter/reseeding.rs:88:1
[00:04:49]    |
[00:04:49] 88 | / pub struct ReseedingRng<R, Rsdr>(BlockRng<ReseedingCore<R, Rsdr>>)
[00:04:49] 89 | | where R: BlockRngCore + SeedableRng,
[00:04:49] 90 | |       Rsdr: RngCore;
[00:04:49]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:04:49] 
[00:04:49] 
[00:04:49] error[E0277]: the trait bound `rand_hc::Hc128Core: rand_core::block::BlockRngCore` is not satisfied
[00:04:49]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/thread.rs:76:1
[00:04:49] 76 | / thread_local!(
[00:04:49] 76 | / thread_local!(
[00:04:49] 77 | |     static THREAD_RNG_KEY: UnsafeCell<ReseedingRng<Hc128Core, EntropyRng>> = {
[00:04:49] 78 | |         let mut entropy_source = EntropyRng::new();
[00:04:49] 79 | |         let r = Hc128Core::from_rng(&mut entropy_source).unwrap_or_else(|err|
[00:04:49] 85 | |     }
[00:04:49] 86 | | );
[00:04:49] 86 | | );
[00:04:49]    | |__^ the trait `rand_core::block::BlockRngCore` is not implemented for `rand_hc::Hc128Core`
[00:04:49]    |
[00:04:49] note: required by `rngs::adapter::reseeding::ReseedingRng`
[00:04:49]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/adapter/reseeding.rs:88:1
[00:04:49]    |
[00:04:49] 88 | / pub struct ReseedingRng<R, Rsdr>(BlockRng<ReseedingCore<R, Rsdr>>)
[00:04:49] 89 | | where R: BlockRngCore + SeedableRng,
[00:04:49] 90 | |       Rsdr: RngCore;
[00:04:49]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:04:49] 
[00:04:49] 
[00:04:49] error[E0277]: the trait bound `rand_hc::Hc128Core: rand_core::SeedableRng` is not satisfied
[00:04:49]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/thread.rs:76:1
[00:04:49] 76 | / thread_local!(
[00:04:49] 76 | / thread_local!(
[00:04:49] 77 | |     static THREAD_RNG_KEY: UnsafeCell<ReseedingRng<Hc128Core, EntropyRng>> = {
[00:04:49] 78 | |         let mut entropy_source = EntropyRng::new();
[00:04:49] 79 | |         let r = Hc128Core::from_rng(&mut entropy_source).unwrap_or_else(|err|
[00:04:49] 85 | |     }
[00:04:49] 86 | | );
[00:04:49] 86 | | );
[00:04:49]    | |__^ the trait `rand_core::SeedableRng` is not implemented for `rand_hc::Hc128Core`
[00:04:49]    |
[00:04:49] note: required by `rngs::adapter::reseeding::ReseedingRng`
[00:04:49]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/adapter/reseeding.rs:88:1
[00:04:49]    |
[00:04:49] 88 | / pub struct ReseedingRng<R, Rsdr>(BlockRng<ReseedingCore<R, Rsdr>>)
[00:04:49] 89 | | where R: BlockRngCore + SeedableRng,
[00:04:49] 90 | |       Rsdr: RngCore;
[00:04:49]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:04:49] 
[00:04:49] 
[00:04:49] error[E0277]: the trait bound `rand_hc::Hc128Core: rand_core::block::BlockRngCore` is not satisfied
[00:04:49]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/thread.rs:76:1
[00:04:49] 76 | / thread_local!(
[00:04:49] 76 | / thread_local!(
[00:04:49] 77 | |     static THREAD_RNG_KEY: UnsafeCell<ReseedingRng<Hc128Core, EntropyRng>> = {
[00:04:49] 78 | |         let mut entropy_source = EntropyRng::new();
[00:04:49] 79 | |         let r = Hc128Core::from_rng(&mut entropy_source).unwrap_or_else(|err|
[00:04:49] 85 | |     }
[00:04:49] 86 | | );
[00:04:49] 86 | | );
[00:04:49]    | |__^ the trait `rand_core::block::BlockRngCore` is not implemented for `rand_hc::Hc128Core`
[00:04:49]    |
[00:04:49] note: required by `rngs::adapter::reseeding::ReseedingRng`
[00:04:49]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/adapter/reseeding.rs:88:1
[00:04:49]    |
[00:04:49] 88 | / pub struct ReseedingRng<R, Rsdr>(BlockRng<ReseedingCore<R, Rsdr>>)
[00:04:49] 89 | | where R: BlockRngCore + SeedableRng,
[00:04:49] 90 | |       Rsdr: RngCore;
[00:04:49]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:04:49] 
[00:04:49] 
[00:04:49] error[E0277]: the trait bound `rand_hc::Hc128Core: rand_core::SeedableRng` is not satisfied
[00:04:49]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/thread.rs:76:1
[00:04:49] 76 | / thread_local!(
[00:04:49] 76 | / thread_local!(
[00:04:49] 77 | |     static THREAD_RNG_KEY: UnsafeCell<ReseedingRng<Hc128Core, EntropyRng>> = {
[00:04:49] 78 | |         let mut entropy_source = EntropyRng::new();
[00:04:49] 79 | |         let r = Hc128Core::from_rng(&mut entropy_source).unwrap_or_else(|err|
[00:04:49] 85 | |     }
[00:04:49] 86 | | );
[00:04:49] 86 | | );
[00:04:49]    | |__^ the trait `rand_core::SeedableRng` is not implemented for `rand_hc::Hc128Core`
[00:04:49]    |
[00:04:49] note: required by `rngs::adapter::reseeding::ReseedingRng`
[00:04:49]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/adapter/reseeding.rs:88:1
[00:04:49]    |
[00:04:49] 88 | / pub struct ReseedingRng<R, Rsdr>(BlockRng<ReseedingCore<R, Rsdr>>)
[00:04:49] 89 | | where R: BlockRngCore + SeedableRng,
[00:04:49] 90 | |       Rsdr: RngCore;
[00:04:49]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:04:49] 
[00:04:49] 
[00:04:49] error[E0277]: the trait bound `rand_hc::Hc128Core: rand_core::block::BlockRngCore` is not satisfied
[00:04:49]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/thread.rs:76:1
[00:04:49] 76 | / thread_local!(
[00:04:49] 76 | / thread_local!(
[00:04:49] 77 | |     static THREAD_RNG_KEY: UnsafeCell<ReseedingRng<Hc128Core, EntropyRng>> = {
[00:04:49] 78 | |         let mut entropy_source = EntropyRng::new();
[00:04:49] 79 | |         let r = Hc128Core::from_rng(&mut entropy_source).unwrap_or_else(|err|
[00:04:49] 85 | |     }
[00:04:49] 86 | | );
[00:04:49] 86 | | );
[00:04:49]    | |__^ the trait `rand_core::block::BlockRngCore` is not implemented for `rand_hc::Hc128Core`
[00:04:49]    |
[00:04:49] note: required by `rngs::adapter::reseeding::ReseedingRng`
[00:04:49]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/adapter/reseeding.rs:88:1
[00:04:49]    |
[00:04:49] 88 | / pub struct ReseedingRng<R, Rsdr>(BlockRng<ReseedingCore<R, Rsdr>>)
[00:04:49] 89 | | where R: BlockRngCore + SeedableRng,
[00:04:49] 90 | |       Rsdr: RngCore;
[00:04:49]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:04:49] 
[00:04:49] 
[00:04:49] error[E0277]: the trait bound `rand_hc::Hc128Core: rand_core::SeedableRng` is not satisfied
[00:04:49]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/thread.rs:76:1
[00:04:49] 76 | / thread_local!(
[00:04:49] 76 | / thread_local!(
[00:04:49] 77 | |     static THREAD_RNG_KEY: UnsafeCell<ReseedingRng<Hc128Core, EntropyRng>> = {
[00:04:49] 78 | |         let mut entropy_source = EntropyRng::new();
[00:04:49] 79 | |         let r = Hc128Core::from_rng(&mut entropy_source).unwrap_or_else(|err|
[00:04:49] 85 | |     }
[00:04:49] 86 | | );
[00:04:49] 86 | | );
[00:04:49]    | |__^ the trait `rand_core::SeedableRng` is not implemented for `rand_hc::Hc128Core`
[00:04:49]    |
[00:04:49] note: required by `rngs::adapter::reseeding::ReseedingRng`
[00:04:49]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/adapter/reseeding.rs:88:1
[00:04:49]    |
[00:04:49] 88 | / pub struct ReseedingRng<R, Rsdr>(BlockRng<ReseedingCore<R, Rsdr>>)
[00:04:49] 89 | | where R: BlockRngCore + SeedableRng,
[00:04:49] 90 | |       Rsdr: RngCore;
[00:04:49]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:04:49] 
[00:04:49] 
[00:04:49] error[E0277]: the trait bound `rand_hc::Hc128Rng: rand_core::SeedableRng` is not satisfied
[00:04:49]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/std.rs:56:5
[00:04:49]    |
[00:04:49] 56 |     type Seed = <Hc128Rng as SeedableRng>::Seed;
[00:04:49]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_hc::Hc128Rng`
[00:04:49] 
[00:04:49] error[E0277]: the trait bound `rand_hc::Hc128Rng: rand_core::SeedableRng` is not satisfied
[00:04:49]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/std.rs:58:5
[00:04:49]    |
[00:04:49] 58 | /     fn from_seed(seed: Self::Seed) -> Self {
[00:04:49] 59 | |         StdRng(Hc128Rng::from_seed(seed))
[00:04:49] 60 | |     }
[00:04:49]    | |_____^ the trait `rand_core::SeedableRng` is not implemented for `rand_hc::Hc128Rng`
[00:04:49] 
[00:04:49] error[E0277]: the trait bound `rand_isaac::IsaacRng: rand_core::SeedableRng` is not satisfied
[00:04:49]    |
[00:04:49]    |
[00:04:49] 51 |     type Seed = <rand_isaac::IsaacRng as SeedableRng>::Seed;
[00:04:49]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_isaac::IsaacRng`
[00:04:49] 
[00:04:49] error[E0277]: the trait bound `rand_isaac::IsaacRng: rand_core::SeedableRng` is not satisfied
[00:04:49]    |
[00:04:49]    |
[00:04:49] 53 | /     fn from_seed(seed: Self::Seed) -> Self {
[00:04:49] 54 | |         IsaacRng(rand_isaac::IsaacRng::from_seed(seed))
[00:04:49] 55 | |     }
[00:04:49]    | |_____^ the trait `rand_core::SeedableRng` is not implemented for `rand_isaac::IsaacRng`
[00:04:49] 
[00:04:49] error[E0277]: the trait bound `rand_isaac::Isaac64Rng: rand_core::SeedableRng` is not satisfied
[00:04:49]    |
[00:04:49]    |
[00:04:49] 97 |     type Seed = <rand_isaac::Isaac64Rng as SeedableRng>::Seed;
[00:04:49]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_isaac::Isaac64Rng`
[00:04:49] 
[00:04:49] error[E0277]: the trait bound `rand_isaac::Isaac64Rng: rand_core::SeedableRng` is not satisfied
[00:04:49]     |
[00:04:49]     |
[00:04:49] 99  | /     fn from_seed(seed: Self::Seed) -> Self {
[00:04:49] 100 | |         Isaac64Rng(rand_isaac::Isaac64Rng::from_seed(seed))
[00:04:49] 101 | |     }
[00:04:49]     | |_____^ the trait `rand_core::SeedableRng` is not implemented for `rand_isaac::Isaac64Rng`
[00:04:49] 
[00:04:49] error[E0277]: the trait bound `rand_hc::Hc128Rng: rand_core::SeedableRng` is not satisfied
[00:04:49]     |
[00:04:49]     |
[00:04:49] 199 |     type Seed = <rand_hc::Hc128Rng as SeedableRng>::Seed;
[00:04:49]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_hc::Hc128Rng`
[00:04:49] 
[00:04:49] error[E0277]: the trait bound `rand_hc::Hc128Rng: rand_core::SeedableRng` is not satisfied
[00:04:49]     |
[00:04:49]     |
[00:04:49] 201 | /     fn from_seed(seed: Self::Seed) -> Self {
[00:04:49] 202 | |         Hc128Rng(rand_hc::Hc128Rng::from_seed(seed))
[00:04:49] 203 | |     }
[00:04:49]     | |_____^ the trait `rand_core::SeedableRng` is not implemented for `rand_hc::Hc128Rng`
[00:04:49] 
[00:04:49] error[E0277]: the trait bound `rand_xorshift::XorShiftRng: rand_core::SeedableRng` is not satisfied
[00:04:49]     |
[00:04:49]     |
[00:04:49] 240 |     type Seed = <::rand_xorshift::XorShiftRng as SeedableRng>::Seed;
[00:04:49]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_xorshift::XorShiftRng`
[00:04:49] 
[00:04:49] error[E0277]: the trait bound `rand_xorshift::XorShiftRng: rand_core::SeedableRng` is not satisfied
[00:04:49]     |
[00:04:49]     |
[00:04:49] 242 | /     fn from_seed(seed: Self::Seed) -> Self {
[00:04:49] 243 | |         XorShiftRng(::rand_xorshift::XorShiftRng::from_seed(seed))
[00:04:49] 244 | |     }
[00:04:49]     | |_____^ the trait `rand_core::SeedableRng` is not implemented for `rand_xorshift::XorShiftRng`
[00:04:49] 
[00:04:49] error[E0277]: the trait bound `rand_hc::Hc128Rng: rand_core::SeedableRng` is not satisfied
[00:04:49]     |
[00:04:49]     |
[00:04:49] 280 |     type Seed = <rngs::StdRng as SeedableRng>::Seed;
[00:04:49]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_hc::Hc128Rng`
[00:04:49] 
[00:04:49] error[E0277]: the trait bound `rand_hc::Hc128Rng: rand_core::SeedableRng` is not satisfied
[00:04:49]     |
[00:04:49]     |
[00:04:49] 282 | /     fn from_seed(seed: Self::Seed) -> Self {
[00:04:49] 283 | |         StdRng(rngs::StdRng::from_seed(seed))
[00:04:49] 284 | |     }
[00:04:49]     | |_____^ the trait `rand_core::SeedableRng` is not implemented for `rand_hc::Hc128Rng`
[00:04:49] error: aborting due to 28 previous errors
[00:04:49] 
[00:04:49] For more information about this error, try `rustc --explain E0277`.
[00:04:49] error: Could not compile `rand`.
[00:04:49] error: Could not compile `rand`.
[00:04:49] warning: build failed, waiting for other jobs to finish...
[00:04:58] error: failed to compile `cargo-vendor v0.1.22`, intermediate artifacts can be found at `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools`
[00:04:58] Caused by:
[00:04:58]   build failed
[00:04:58] 
[00:04:58] 
[00:04:58] 
[00:04:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "install" "-j" "4" "--locked" "--color" "always" "--force" "--debug" "--vers" "0.1.22" "cargo-vendor"
[00:04:58] 
[00:04:58] 
[00:04:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test distcheck
[00:04:58] Build completed unsuccessfully in 0:00:56
---
travis_time:end:0060dd00:start=1559839411163599317,finish=1559839411170517280,duration=6917963
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2f507960
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:070d5bd6
travis_time:start:070d5bd6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:16bb9a40
$ dmesg | grep -i kill
