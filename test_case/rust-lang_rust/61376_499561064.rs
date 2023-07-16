plain
[00:05:21]    Compiling aho-corasick v0.7.3
[00:05:21]    Compiling bstr v0.1.4
[00:05:22]    Compiling clap v2.33.0
[00:05:23]    Compiling tar v0.4.26
[00:05:25] error[E0277]: the trait bound `rand_hc::Hc128Rng: rand_core::SeedableRng` is not satisfied
[00:05:25]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/std.rs:55:6
[00:05:25]    |
[00:05:25] 55 | impl SeedableRng for StdRng {
[00:05:25]    |      ^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_hc::Hc128Rng`
[00:05:25] 
[00:05:25] error[E0277]: the trait bound `rand_isaac::IsaacRng: rand_core::SeedableRng` is not satisfied
[00:05:25]    |
[00:05:25]    |
[00:05:25] 50 | impl SeedableRng for IsaacRng {
[00:05:25]    |      ^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_isaac::IsaacRng`
[00:05:25] 
[00:05:25] error[E0277]: the trait bound `rand_isaac::Isaac64Rng: rand_core::SeedableRng` is not satisfied
[00:05:25]    |
[00:05:25]    |
[00:05:25] 96 | impl SeedableRng for Isaac64Rng {
[00:05:25]    |      ^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_isaac::Isaac64Rng`
[00:05:25] 
[00:05:25] error[E0277]: the trait bound `rand_hc::Hc128Rng: rand_core::SeedableRng` is not satisfied
[00:05:25]     |
[00:05:25]     |
[00:05:25] 198 | impl SeedableRng for Hc128Rng {
[00:05:25]     |      ^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_hc::Hc128Rng`
[00:05:25] 
[00:05:25] error[E0277]: the trait bound `rand_xorshift::XorShiftRng: rand_core::SeedableRng` is not satisfied
[00:05:25]     |
[00:05:25]     |
[00:05:25] 239 | impl SeedableRng for XorShiftRng {
[00:05:25]     |      ^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_xorshift::XorShiftRng`
[00:05:25] 
[00:05:25] error[E0277]: the trait bound `rand_hc::Hc128Rng: rand_core::SeedableRng` is not satisfied
[00:05:25]     |
[00:05:25]     |
[00:05:25] 279 | impl SeedableRng for StdRng {
[00:05:25]     |      ^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_hc::Hc128Rng`
[00:05:25] 
[00:05:25] error[E0277]: the trait bound `rand_hc::Hc128Core: rand_core::block::BlockRngCore` is not satisfied
[00:05:25]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/thread.rs:73:5
[00:05:25]    |
[00:05:25] 73 |     rng: *mut ReseedingRng<Hc128Core, EntropyRng>,
[00:05:25]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rand_core::block::BlockRngCore` is not implemented for `rand_hc::Hc128Core`
[00:05:25]    |
[00:05:25] note: required by `rngs::adapter::reseeding::ReseedingRng`
[00:05:25]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/adapter/reseeding.rs:88:1
[00:05:25]    |
[00:05:25] 88 | / pub struct ReseedingRng<R, Rsdr>(BlockRng<ReseedingCore<R, Rsdr>>)
[00:05:25] 89 | | where R: BlockRngCore + SeedableRng,
[00:05:25] 90 | |       Rsdr: RngCore;
[00:05:25] 
[00:05:25] 
[00:05:25] error[E0277]: the trait bound `rand_hc::Hc128Core: rand_core::SeedableRng` is not satisfied
[00:05:25]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/thread.rs:73:5
[00:05:25]    |
[00:05:25] 73 |     rng: *mut ReseedingRng<Hc128Core, EntropyRng>,
[00:05:25]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_hc::Hc128Core`
[00:05:25]    |
[00:05:25] note: required by `rngs::adapter::reseeding::ReseedingRng`
[00:05:25]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/adapter/reseeding.rs:88:1
[00:05:25]    |
[00:05:25] 88 | / pub struct ReseedingRng<R, Rsdr>(BlockRng<ReseedingCore<R, Rsdr>>)
[00:05:25] 89 | | where R: BlockRngCore + SeedableRng,
[00:05:25] 90 | |       Rsdr: RngCore;
[00:05:25] 
[00:05:25] 
[00:05:25] error[E0277]: the trait bound `rand_hc::Hc128Core: rand_core::block::BlockRngCore` is not satisfied
[00:05:25]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/thread.rs:76:1
[00:05:25] 76 | / thread_local!(
[00:05:25] 76 | / thread_local!(
[00:05:25] 77 | |     static THREAD_RNG_KEY: UnsafeCell<ReseedingRng<Hc128Core, EntropyRng>> = {
[00:05:25] 78 | |         let mut entropy_source = EntropyRng::new();
[00:05:25] 79 | |         let r = Hc128Core::from_rng(&mut entropy_source).unwrap_or_else(|err|
[00:05:25] 85 | |     }
[00:05:25] 86 | | );
[00:05:25] 86 | | );
[00:05:25]    | |__^ the trait `rand_core::block::BlockRngCore` is not implemented for `rand_hc::Hc128Core`
[00:05:25]    |
[00:05:25] note: required by `rngs::adapter::reseeding::ReseedingRng`
[00:05:25]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/adapter/reseeding.rs:88:1
[00:05:25]    |
[00:05:25] 88 | / pub struct ReseedingRng<R, Rsdr>(BlockRng<ReseedingCore<R, Rsdr>>)
[00:05:25] 89 | | where R: BlockRngCore + SeedableRng,
[00:05:25] 90 | |       Rsdr: RngCore;
[00:05:25]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:05:25] 
[00:05:25] 
[00:05:25] error[E0277]: the trait bound `rand_hc::Hc128Core: rand_core::SeedableRng` is not satisfied
[00:05:25]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/thread.rs:76:1
[00:05:25] 76 | / thread_local!(
[00:05:25] 76 | / thread_local!(
[00:05:25] 77 | |     static THREAD_RNG_KEY: UnsafeCell<ReseedingRng<Hc128Core, EntropyRng>> = {
[00:05:25] 78 | |         let mut entropy_source = EntropyRng::new();
[00:05:25] 79 | |         let r = Hc128Core::from_rng(&mut entropy_source).unwrap_or_else(|err|
[00:05:25] 85 | |     }
[00:05:25] 86 | | );
[00:05:25] 86 | | );
[00:05:25]    | |__^ the trait `rand_core::SeedableRng` is not implemented for `rand_hc::Hc128Core`
[00:05:25]    |
[00:05:25] note: required by `rngs::adapter::reseeding::ReseedingRng`
[00:05:25]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/adapter/reseeding.rs:88:1
[00:05:25]    |
[00:05:25] 88 | / pub struct ReseedingRng<R, Rsdr>(BlockRng<ReseedingCore<R, Rsdr>>)
[00:05:25] 89 | | where R: BlockRngCore + SeedableRng,
[00:05:25] 90 | |       Rsdr: RngCore;
[00:05:25]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:05:25] 
[00:05:25] 
[00:05:25] error[E0277]: the trait bound `rand_hc::Hc128Core: rand_core::block::BlockRngCore` is not satisfied
[00:05:25]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/thread.rs:76:1
[00:05:25] 76 | / thread_local!(
[00:05:25] 76 | / thread_local!(
[00:05:25] 77 | |     static THREAD_RNG_KEY: UnsafeCell<ReseedingRng<Hc128Core, EntropyRng>> = {
[00:05:25] 78 | |         let mut entropy_source = EntropyRng::new();
[00:05:25] 79 | |         let r = Hc128Core::from_rng(&mut entropy_source).unwrap_or_else(|err|
[00:05:25] 85 | |     }
[00:05:25] 86 | | );
[00:05:25] 86 | | );
[00:05:25]    | |__^ the trait `rand_core::block::BlockRngCore` is not implemented for `rand_hc::Hc128Core`
[00:05:25]    |
[00:05:25] note: required by `rngs::adapter::reseeding::ReseedingRng`
[00:05:25]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/adapter/reseeding.rs:88:1
[00:05:25]    |
[00:05:25] 88 | / pub struct ReseedingRng<R, Rsdr>(BlockRng<ReseedingCore<R, Rsdr>>)
[00:05:25] 89 | | where R: BlockRngCore + SeedableRng,
[00:05:25] 90 | |       Rsdr: RngCore;
[00:05:25]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:05:25] 
[00:05:25] 
[00:05:25] error[E0277]: the trait bound `rand_hc::Hc128Core: rand_core::SeedableRng` is not satisfied
[00:05:25]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/thread.rs:76:1
[00:05:25] 76 | / thread_local!(
[00:05:25] 76 | / thread_local!(
[00:05:25] 77 | |     static THREAD_RNG_KEY: UnsafeCell<ReseedingRng<Hc128Core, EntropyRng>> = {
[00:05:25] 78 | |         let mut entropy_source = EntropyRng::new();
[00:05:25] 79 | |         let r = Hc128Core::from_rng(&mut entropy_source).unwrap_or_else(|err|
[00:05:25] 85 | |     }
[00:05:25] 86 | | );
[00:05:25] 86 | | );
[00:05:25]    | |__^ the trait `rand_core::SeedableRng` is not implemented for `rand_hc::Hc128Core`
[00:05:25]    |
[00:05:25] note: required by `rngs::adapter::reseeding::ReseedingRng`
[00:05:25]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/adapter/reseeding.rs:88:1
[00:05:25]    |
[00:05:25] 88 | / pub struct ReseedingRng<R, Rsdr>(BlockRng<ReseedingCore<R, Rsdr>>)
[00:05:25] 89 | | where R: BlockRngCore + SeedableRng,
[00:05:25] 90 | |       Rsdr: RngCore;
[00:05:25]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:05:25] 
[00:05:25] 
[00:05:25] error[E0277]: the trait bound `rand_hc::Hc128Core: rand_core::block::BlockRngCore` is not satisfied
[00:05:25]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/thread.rs:76:1
[00:05:25] 76 | / thread_local!(
[00:05:25] 76 | / thread_local!(
[00:05:25] 77 | |     static THREAD_RNG_KEY: UnsafeCell<ReseedingRng<Hc128Core, EntropyRng>> = {
[00:05:25] 78 | |         let mut entropy_source = EntropyRng::new();
[00:05:25] 79 | |         let r = Hc128Core::from_rng(&mut entropy_source).unwrap_or_else(|err|
[00:05:25] 85 | |     }
[00:05:25] 86 | | );
[00:05:25] 86 | | );
[00:05:25]    | |__^ the trait `rand_core::block::BlockRngCore` is not implemented for `rand_hc::Hc128Core`
[00:05:25]    |
[00:05:25] note: required by `rngs::adapter::reseeding::ReseedingRng`
[00:05:25]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/adapter/reseeding.rs:88:1
[00:05:25]    |
[00:05:25] 88 | / pub struct ReseedingRng<R, Rsdr>(BlockRng<ReseedingCore<R, Rsdr>>)
[00:05:25] 89 | | where R: BlockRngCore + SeedableRng,
[00:05:25] 90 | |       Rsdr: RngCore;
[00:05:25]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:05:25] 
[00:05:25] 
[00:05:25] error[E0277]: the trait bound `rand_hc::Hc128Core: rand_core::SeedableRng` is not satisfied
[00:05:25]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/thread.rs:76:1
[00:05:25] 76 | / thread_local!(
[00:05:25] 76 | / thread_local!(
[00:05:25] 77 | |     static THREAD_RNG_KEY: UnsafeCell<ReseedingRng<Hc128Core, EntropyRng>> = {
[00:05:25] 78 | |         let mut entropy_source = EntropyRng::new();
[00:05:25] 79 | |         let r = Hc128Core::from_rng(&mut entropy_source).unwrap_or_else(|err|
[00:05:25] 85 | |     }
[00:05:25] 86 | | );
[00:05:25] 86 | | );
[00:05:25]    | |__^ the trait `rand_core::SeedableRng` is not implemented for `rand_hc::Hc128Core`
[00:05:25]    |
[00:05:25] note: required by `rngs::adapter::reseeding::ReseedingRng`
[00:05:25]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/adapter/reseeding.rs:88:1
[00:05:25]    |
[00:05:25] 88 | / pub struct ReseedingRng<R, Rsdr>(BlockRng<ReseedingCore<R, Rsdr>>)
[00:05:25] 89 | | where R: BlockRngCore + SeedableRng,
[00:05:25] 90 | |       Rsdr: RngCore;
[00:05:25]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:05:25] 
[00:05:25] 
[00:05:25] error[E0277]: the trait bound `rand_hc::Hc128Core: rand_core::block::BlockRngCore` is not satisfied
[00:05:25]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/thread.rs:76:1
[00:05:25] 76 | / thread_local!(
[00:05:25] 76 | / thread_local!(
[00:05:25] 77 | |     static THREAD_RNG_KEY: UnsafeCell<ReseedingRng<Hc128Core, EntropyRng>> = {
[00:05:25] 78 | |         let mut entropy_source = EntropyRng::new();
[00:05:25] 79 | |         let r = Hc128Core::from_rng(&mut entropy_source).unwrap_or_else(|err|
[00:05:25] 85 | |     }
[00:05:25] 86 | | );
[00:05:25] 86 | | );
[00:05:25]    | |__^ the trait `rand_core::block::BlockRngCore` is not implemented for `rand_hc::Hc128Core`
[00:05:25]    |
[00:05:25] note: required by `rngs::adapter::reseeding::ReseedingRng`
[00:05:25]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/adapter/reseeding.rs:88:1
[00:05:25]    |
[00:05:25] 88 | / pub struct ReseedingRng<R, Rsdr>(BlockRng<ReseedingCore<R, Rsdr>>)
[00:05:25] 89 | | where R: BlockRngCore + SeedableRng,
[00:05:25] 90 | |       Rsdr: RngCore;
[00:05:25]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:05:25] 
[00:05:25] 
[00:05:25] error[E0277]: the trait bound `rand_hc::Hc128Core: rand_core::SeedableRng` is not satisfied
[00:05:25]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/thread.rs:76:1
[00:05:25] 76 | / thread_local!(
[00:05:25] 76 | / thread_local!(
[00:05:25] 77 | |     static THREAD_RNG_KEY: UnsafeCell<ReseedingRng<Hc128Core, EntropyRng>> = {
[00:05:25] 78 | |         let mut entropy_source = EntropyRng::new();
[00:05:25] 79 | |         let r = Hc128Core::from_rng(&mut entropy_source).unwrap_or_else(|err|
[00:05:25] 85 | |     }
[00:05:25] 86 | | );
[00:05:25] 86 | | );
[00:05:25]    | |__^ the trait `rand_core::SeedableRng` is not implemented for `rand_hc::Hc128Core`
[00:05:25]    |
[00:05:25] note: required by `rngs::adapter::reseeding::ReseedingRng`
[00:05:25]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/adapter/reseeding.rs:88:1
[00:05:25]    |
[00:05:25] 88 | / pub struct ReseedingRng<R, Rsdr>(BlockRng<ReseedingCore<R, Rsdr>>)
[00:05:25] 89 | | where R: BlockRngCore + SeedableRng,
[00:05:25] 90 | |       Rsdr: RngCore;
[00:05:25]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:05:25] 
[00:05:25] 
[00:05:25] error[E0277]: the trait bound `rand_hc::Hc128Rng: rand_core::SeedableRng` is not satisfied
[00:05:25]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/std.rs:56:5
[00:05:25]    |
[00:05:25] 56 |     type Seed = <Hc128Rng as SeedableRng>::Seed;
[00:05:25]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_hc::Hc128Rng`
[00:05:25] 
[00:05:25] error[E0277]: the trait bound `rand_hc::Hc128Rng: rand_core::SeedableRng` is not satisfied
[00:05:25]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/std.rs:58:5
[00:05:25]    |
[00:05:25] 58 | /     fn from_seed(seed: Self::Seed) -> Self {
[00:05:25] 59 | |         StdRng(Hc128Rng::from_seed(seed))
[00:05:25] 60 | |     }
[00:05:25]    | |_____^ the trait `rand_core::SeedableRng` is not implemented for `rand_hc::Hc128Rng`
[00:05:25] 
[00:05:25] error[E0277]: the trait bound `rand_isaac::IsaacRng: rand_core::SeedableRng` is not satisfied
[00:05:25]    |
[00:05:25]    |
[00:05:25] 51 |     type Seed = <rand_isaac::IsaacRng as SeedableRng>::Seed;
[00:05:25]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_isaac::IsaacRng`
[00:05:25] 
[00:05:25] error[E0277]: the trait bound `rand_isaac::IsaacRng: rand_core::SeedableRng` is not satisfied
[00:05:25]    |
[00:05:25]    |
[00:05:25] 53 | /     fn from_seed(seed: Self::Seed) -> Self {
[00:05:25] 54 | |         IsaacRng(rand_isaac::IsaacRng::from_seed(seed))
[00:05:25] 55 | |     }
[00:05:25]    | |_____^ the trait `rand_core::SeedableRng` is not implemented for `rand_isaac::IsaacRng`
[00:05:25] 
[00:05:25] error[E0277]: the trait bound `rand_isaac::Isaac64Rng: rand_core::SeedableRng` is not satisfied
[00:05:25]    |
[00:05:25]    |
[00:05:25] 97 |     type Seed = <rand_isaac::Isaac64Rng as SeedableRng>::Seed;
[00:05:25]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_isaac::Isaac64Rng`
[00:05:25] 
[00:05:25] error[E0277]: the trait bound `rand_isaac::Isaac64Rng: rand_core::SeedableRng` is not satisfied
[00:05:25]     |
[00:05:25]     |
[00:05:25] 99  | /     fn from_seed(seed: Self::Seed) -> Self {
[00:05:25] 100 | |         Isaac64Rng(rand_isaac::Isaac64Rng::from_seed(seed))
[00:05:25] 101 | |     }
[00:05:25]     | |_____^ the trait `rand_core::SeedableRng` is not implemented for `rand_isaac::Isaac64Rng`
[00:05:25] 
[00:05:25] error[E0277]: the trait bound `rand_hc::Hc128Rng: rand_core::SeedableRng` is not satisfied
[00:05:25]     |
[00:05:25]     |
[00:05:25] 199 |     type Seed = <rand_hc::Hc128Rng as SeedableRng>::Seed;
[00:05:25]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_hc::Hc128Rng`
[00:05:25] 
[00:05:25] error[E0277]: the trait bound `rand_hc::Hc128Rng: rand_core::SeedableRng` is not satisfied
[00:05:25]     |
[00:05:25]     |
[00:05:25] 201 | /     fn from_seed(seed: Self::Seed) -> Self {
[00:05:25] 202 | |         Hc128Rng(rand_hc::Hc128Rng::from_seed(seed))
[00:05:25] 203 | |     }
[00:05:25]     | |_____^ the trait `rand_core::SeedableRng` is not implemented for `rand_hc::Hc128Rng`
[00:05:25] 
[00:05:25] error[E0277]: the trait bound `rand_xorshift::XorShiftRng: rand_core::SeedableRng` is not satisfied
[00:05:25]     |
[00:05:25]     |
[00:05:25] 240 |     type Seed = <::rand_xorshift::XorShiftRng as SeedableRng>::Seed;
[00:05:25]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_xorshift::XorShiftRng`
[00:05:25] 
[00:05:25] error[E0277]: the trait bound `rand_xorshift::XorShiftRng: rand_core::SeedableRng` is not satisfied
[00:05:25]     |
[00:05:25]     |
[00:05:25] 242 | /     fn from_seed(seed: Self::Seed) -> Self {
[00:05:25] 243 | |         XorShiftRng(::rand_xorshift::XorShiftRng::from_seed(seed))
[00:05:25] 244 | |     }
[00:05:25]     | |_____^ the trait `rand_core::SeedableRng` is not implemented for `rand_xorshift::XorShiftRng`
[00:05:25] 
[00:05:25] error[E0277]: the trait bound `rand_hc::Hc128Rng: rand_core::SeedableRng` is not satisfied
[00:05:25]     |
[00:05:25]     |
[00:05:25] 280 |     type Seed = <rngs::StdRng as SeedableRng>::Seed;
[00:05:25]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_hc::Hc128Rng`
[00:05:25] 
[00:05:25] error[E0277]: the trait bound `rand_hc::Hc128Rng: rand_core::SeedableRng` is not satisfied
[00:05:25]     |
[00:05:25]     |
[00:05:25] 282 | /     fn from_seed(seed: Self::Seed) -> Self {
[00:05:25] 283 | |         StdRng(rngs::StdRng::from_seed(seed))
[00:05:25] 284 | |     }
[00:05:25]     | |_____^ the trait `rand_core::SeedableRng` is not implemented for `rand_hc::Hc128Rng`
[00:05:25]    Compiling url v1.7.2
[00:05:25] error: aborting due to 28 previous errors
[00:05:25] 
[00:05:25] For more information about this error, try `rustc --explain E0277`.
[00:05:25] For more information about this error, try `rustc --explain E0277`.
[00:05:25] error: Could not compile `rand`.
[00:05:25] warning: build failed, waiting for other jobs to finish...
[00:05:31] error: failed to compile `cargo-vendor v0.1.22`, intermediate artifacts can be found at `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools`
[00:05:31] Caused by:
[00:05:31]   build failed
[00:05:31] 
[00:05:31] 
[00:05:31] 
[00:05:31] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "install" "-j" "4" "--locked" "--color" "always" "--force" "--debug" "--vers" "0.1.22" "cargo-vendor"
[00:05:31] 
[00:05:31] 
[00:05:31] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test distcheck
[00:05:31] Build completed unsuccessfully in 0:00:43
---
travis_time:end:163eb348:start=1559837144352244760,finish=1559837144359472503,duration=7227743
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03825656
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01bd3052
travis_time:start:01bd3052
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2044f371
$ dmesg | grep -i kill
