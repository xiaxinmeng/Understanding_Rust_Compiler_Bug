plain
travis_time:end:0bbe82fe:start=1555081172235775676,finish=1555081173011138189,duration=775362513
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:05:42]    Compiling env_logger v0.5.13
[00:05:44]    Compiling backtrace v0.3.11
[00:05:44]    Compiling flate2 v1.0.6
[00:05:45]    Compiling syn v0.15.22
[00:05:47] error[E0277]: the trait bound `rand_pcg::Mcg128Xsl64: rand_core::SeedableRng` is not satisfied
[00:05:47]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/small.rs:96:6
[00:05:47]    |
[00:05:47] 96 | impl SeedableRng for SmallRng {
[00:05:47]    |      ^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_pcg::Mcg128Xsl64`
[00:05:47] 
[00:05:47] error[E0277]: the trait bound `rand_hc::Hc128Rng: rand_core::SeedableRng` is not satisfied
[00:05:47]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/std.rs:55:6
[00:05:47]    |
[00:05:47] 55 | impl SeedableRng for StdRng {
[00:05:47]    |      ^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_hc::Hc128Rng`
[00:05:47] 
[00:05:47] error[E0277]: the trait bound `rand_isaac::IsaacRng: rand_core::SeedableRng` is not satisfied
[00:05:47]    |
[00:05:47]    |
[00:05:47] 50 | impl SeedableRng for IsaacRng {
[00:05:47]    |      ^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_isaac::IsaacRng`
[00:05:47] 
[00:05:47] error[E0277]: the trait bound `rand_isaac::Isaac64Rng: rand_core::SeedableRng` is not satisfied
[00:05:47]    |
[00:05:47]    |
[00:05:47] 96 | impl SeedableRng for Isaac64Rng {
[00:05:47]    |      ^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_isaac::Isaac64Rng`
[00:05:47] 
[00:05:47] error[E0277]: the trait bound `rand_chacha::ChaChaRng: rand_core::SeedableRng` is not satisfied
[00:05:47]     |
[00:05:47]     |
[00:05:47] 141 | impl SeedableRng for ChaChaRng {
[00:05:47]     |      ^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_chacha::ChaChaRng`
[00:05:47] 
[00:05:47] error[E0277]: the trait bound `rand_hc::Hc128Rng: rand_core::SeedableRng` is not satisfied
[00:05:47]     |
[00:05:47]     |
[00:05:47] 198 | impl SeedableRng for Hc128Rng {
[00:05:47]     |      ^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_hc::Hc128Rng`
[00:05:47] 
[00:05:47] error[E0277]: the trait bound `rand_xorshift::XorShiftRng: rand_core::SeedableRng` is not satisfied
[00:05:47]     |
[00:05:47]     |
[00:05:47] 239 | impl SeedableRng for XorShiftRng {
[00:05:47]     |      ^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_xorshift::XorShiftRng`
[00:05:47] 
[00:05:47] error[E0277]: the trait bound `rand_hc::Hc128Rng: rand_core::SeedableRng` is not satisfied
[00:05:47]     |
[00:05:47]     |
[00:05:47] 279 | impl SeedableRng for StdRng {
[00:05:47]     |      ^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_hc::Hc128Rng`
[00:05:47] 
[00:05:47] error[E0277]: the trait bound `rand_hc::Hc128Core: rand_core::block::BlockRngCore` is not satisfied
[00:05:47]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/thread.rs:73:5
[00:05:47]    |
[00:05:47] 73 |     rng: *mut ReseedingRng<Hc128Core, EntropyRng>,
[00:05:47]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rand_core::block::BlockRngCore` is not implemented for `rand_hc::Hc128Core`
[00:05:47]    |
[00:05:47] note: required by `rngs::adapter::reseeding::ReseedingRng`
[00:05:47]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/adapter/reseeding.rs:88:1
[00:05:47]    |
[00:05:47] 88 | / pub struct ReseedingRng<R, Rsdr>(BlockRng<ReseedingCore<R, Rsdr>>)
[00:05:47] 89 | | where R: BlockRngCore + SeedableRng,
[00:05:47] 90 | |       Rsdr: RngCore;
[00:05:47] 
[00:05:47] 
[00:05:47] error[E0277]: the trait bound `rand_hc::Hc128Core: rand_core::SeedableRng` is not satisfied
[00:05:47]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/thread.rs:73:5
[00:05:47]    |
[00:05:47] 73 |     rng: *mut ReseedingRng<Hc128Core, EntropyRng>,
[00:05:47]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_hc::Hc128Core`
[00:05:47]    |
[00:05:47] note: required by `rngs::adapter::reseeding::ReseedingRng`
[00:05:47]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/adapter/reseeding.rs:88:1
[00:05:47]    |
[00:05:47] 88 | / pub struct ReseedingRng<R, Rsdr>(BlockRng<ReseedingCore<R, Rsdr>>)
[00:05:47] 89 | | where R: BlockRngCore + SeedableRng,
[00:05:47] 90 | |       Rsdr: RngCore;
[00:05:47] 
[00:05:47] 
[00:05:47] error[E0277]: the trait bound `rand_hc::Hc128Core: rand_core::block::BlockRngCore` is not satisfied
[00:05:47]   ::: <::std::thread::local::thread_local macros>:1:1
[00:05:47]    |
[00:05:47] 1  |   / (  ) => {  } ; (
[00:05:47] 1  |   / (  ) => {  } ; (
[00:05:47] 2  |   | $ ( # [ $ attr : meta ] ) * $ vis : vis static $ name : ident : $ t : ty = $
[00:05:47] 3  |   | init : expr ; $ ( $ rest : tt ) * ) => (
[00:05:47] 4  |   | $ crate :: __thread_local_inner ! (
[00:05:47] ...    |
[00:05:47] 9  |   | $ crate :: __thread_local_inner ! (
[00:05:47]    |  _|_-
[00:05:47] 10 | | | $ ( # [ $ attr ] ) * $ vis $ name , $ t , $ init ) ; ) ;
[00:05:47]    | | |____________________________________________________-___- in this expansion of `thread_local!`
[00:05:47]    |                                                        in this macro invocation
[00:05:47]    |                                                        in this macro invocation
[00:05:47]   --> <::std::thread::local::__thread_local_inner macros>:27:43
[00:05:47] 1  |  /  (
[00:05:47] 1  |  /  (
[00:05:47] 2  |  |  @ key $ ( # [ $ attr : meta ] ) * $ vis : vis $ name : ident , $ t : ty , $
[00:05:47] 3  |  |  init : expr ) => {
[00:05:47] 4  |  |  {
[00:05:47] ...   |
[00:05:47] 27 |  |  $ ( # [ $ attr ] ) * $ vis const $ name : $ crate :: thread :: LocalKey < $ t
[00:05:47]    |  |____________________________________________^
[00:05:47] 28 | ||  > = $ crate :: __thread_local_inner ! (
[00:05:47]    | ||__^ the trait `rand_core::block::BlockRngCore` is not implemented for `rand_hc::Hc128Core`
[00:05:47] 29 |  |  @ key $ ( # [ $ attr ] ) * $ vis $ name , $ t , $ init ) ; }
[00:05:47]    |  |_____________________________________________________________- in this expansion of `$crate::__thread_local_inner!`
[00:05:47]    | 
[00:05:47]   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/thread.rs:76:1
[00:05:47] 76 | /   thread_local!(
[00:05:47] 76 | /   thread_local!(
[00:05:47] 77 | |       static THREAD_RNG_KEY: UnsafeCell<ReseedingRng<Hc128Core, EntropyRng>> = {
[00:05:47] 78 | |           let mut entropy_source = EntropyRng::new();
[00:05:47] 79 | |           let r = Hc128Core::from_rng(&mut entropy_source).unwrap_or_else(|err|
[00:05:47] 85 | |       }
[00:05:47] 86 | |   );
[00:05:47]    | |____- in this macro invocation
[00:05:47]    |
[00:05:47]    |
[00:05:47] note: required by `rngs::adapter::reseeding::ReseedingRng`
[00:05:47]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/adapter/reseeding.rs:88:1
[00:05:47]    |
[00:05:47] 88 | / pub struct ReseedingRng<R, Rsdr>(BlockRng<ReseedingCore<R, Rsdr>>)
[00:05:47] 89 | | where R: BlockRngCore + SeedableRng,
[00:05:47] 90 | |       Rsdr: RngCore;
[00:05:47] 
[00:05:47] 
[00:05:47] error[E0277]: the trait bound `rand_hc::Hc128Core: rand_core::SeedableRng` is not satisfied
[00:05:47]   ::: <::std::thread::local::thread_local macros>:1:1
[00:05:47]    |
[00:05:47] 1  |   / (  ) => {  } ; (
[00:05:47] 1  |   / (  ) => {  } ; (
[00:05:47] 2  |   | $ ( # [ $ attr : meta ] ) * $ vis : vis static $ name : ident : $ t : ty = $
[00:05:47] 3  |   | init : expr ; $ ( $ rest : tt ) * ) => (
[00:05:47] 4  |   | $ crate :: __thread_local_inner ! (
[00:05:47] ...    |
[00:05:47] 9  |   | $ crate :: __thread_local_inner ! (
[00:05:47]    |  _|_-
[00:05:47] 10 | | | $ ( # [ $ attr ] ) * $ vis $ name , $ t , $ init ) ; ) ;
[00:05:47]    | | |____________________________________________________-___- in this expansion of `thread_local!`
[00:05:47]    |                                                        in this macro invocation
[00:05:47]    |                                                        in this macro invocation
[00:05:47]   --> <::std::thread::local::__thread_local_inner macros>:27:43
[00:05:47] 1  |  /  (
[00:05:47] 1  |  /  (
[00:05:47] 2  |  |  @ key $ ( # [ $ attr : meta ] ) * $ vis : vis $ name : ident , $ t : ty , $
[00:05:47] 3  |  |  init : expr ) => {
[00:05:47] 4  |  |  {
[00:05:47] ...   |
[00:05:47] 27 |  |  $ ( # [ $ attr ] ) * $ vis const $ name : $ crate :: thread :: LocalKey < $ t
[00:05:47]    |  |____________________________________________^
[00:05:47] 28 | ||  > = $ crate :: __thread_local_inner ! (
[00:05:47]    | ||__^ the trait `rand_core::SeedableRng` is not implemented for `rand_hc::Hc128Core`
[00:05:47] 29 |  |  @ key $ ( # [ $ attr ] ) * $ vis $ name , $ t , $ init ) ; }
[00:05:47]    |  |_____________________________________________________________- in this expansion of `$crate::__thread_local_inner!`
[00:05:47]    | 
[00:05:47]   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/thread.rs:76:1
[00:05:47] 76 | /   thread_local!(
[00:05:47] 76 | /   thread_local!(
[00:05:47] 77 | |       static THREAD_RNG_KEY: UnsafeCell<ReseedingRng<Hc128Core, EntropyRng>> = {
[00:05:47] 78 | |           let mut entropy_source = EntropyRng::new();
[00:05:47] 79 | |           let r = Hc128Core::from_rng(&mut entropy_source).unwrap_or_else(|err|
[00:05:47] 85 | |       }
[00:05:47] 86 | |   );
[00:05:47]    | |____- in this macro invocation
[00:05:47]    |
[00:05:47]    |
[00:05:47] note: required by `rngs::adapter::reseeding::ReseedingRng`
[00:05:47]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/adapter/reseeding.rs:88:1
[00:05:47]    |
[00:05:47] 88 | / pub struct ReseedingRng<R, Rsdr>(BlockRng<ReseedingCore<R, Rsdr>>)
[00:05:47] 89 | | where R: BlockRngCore + SeedableRng,
[00:05:47] 90 | |       Rsdr: RngCore;
[00:05:47] 
[00:05:47] 
[00:05:47] error[E0277]: the trait bound `rand_hc::Hc128Core: rand_core::block::BlockRngCore` is not satisfied
[00:05:47]   --> <::std::thread::local::__thread_local_inner macros>:5:14
[00:05:47] 1  |      (
[00:05:47]    |    __-
[00:05:47]    |   |__|
[00:05:47]    |  ||
[00:05:47]    |  ||
[00:05:47] 2  |  ||  @ key $ ( # [ $ attr : meta ] ) * $ vis : vis $ name : ident , $ t : ty , $
[00:05:47] 3  |  ||  init : expr ) => {
[00:05:47] 4  |  ||  {
[00:05:47] 5  |  ||  # [ inline ] fn __init (  ) -> $ t { $ init } unsafe fn __getit (  ) -> $
[00:05:47]    |  ||               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rand_core::block::BlockRngCore` is not implemented for `rand_hc::Hc128Core`
[00:05:47] ...   ||
[00:05:47] 28 |  ||  > = $ crate :: __thread_local_inner ! (
[00:05:47]    |  ||______-
[00:05:47] 29 | |||  @ key $ ( # [ $ attr ] ) * $ vis $ name , $ t , $ init ) ; }
[00:05:47]    | |||_________________________________________________________|___|
[00:05:47]    | |||_________________________________________________________|___|
[00:05:47]    | ||__________________________________________________________|___in this expansion of `$crate::__thread_local_inner!` (#2)
[00:05:47]    | |___________________________________________________________|   in this expansion of `$crate::__thread_local_inner!` (#3)
[00:05:47]    | 
[00:05:47]   ::: <::std::thread::local::thread_local macros>:1:1
[00:05:47]    |
[00:05:47] 1  |    / (  ) => {  } ; (
[00:05:47] 1  |    / (  ) => {  } ; (
[00:05:47] 2  |    | $ ( # [ $ attr : meta ] ) * $ vis : vis static $ name : ident : $ t : ty = $
[00:05:47] 3  |    | init : expr ; $ ( $ rest : tt ) * ) => (
[00:05:47] 4  |    | $ crate :: __thread_local_inner ! (
[00:05:47] ...     |
[00:05:47] 9  |    | $ crate :: __thread_local_inner ! (
[00:05:47]    |  __|_-
[00:05:47] 10 | |  | $ ( # [ $ attr ] ) * $ vis $ name , $ t , $ init ) ; ) ;
[00:05:47]    | |  |____________________________________________________-___- in this expansion of `thread_local!` (#1)
[00:05:47]    |                                                         in this macro invocation (#2)
[00:05:47]    | 
[00:05:47]    | 
[00:05:47]   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/thread.rs:76:1
[00:05:47] 76 | /    thread_local!(
[00:05:47] 76 | /    thread_local!(
[00:05:47] 77 | |        static THREAD_RNG_KEY: UnsafeCell<ReseedingRng<Hc128Core, EntropyRng>> = {
[00:05:47] 78 | |            let mut entropy_source = EntropyRng::new();
[00:05:47] 79 | |            let r = Hc128Core::from_rng(&mut entropy_source).unwrap_or_else(|err|
[00:05:47] 85 | |        }
[00:05:47] 86 | |    );
[00:05:47]    | |_____- in this macro invocation (#1)
[00:05:47]    |
[00:05:47]    |
[00:05:47] note: required by `rngs::adapter::reseeding::ReseedingRng`
[00:05:47]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/adapter/reseeding.rs:88:1
[00:05:47]    |
[00:05:47] 88 | / pub struct ReseedingRng<R, Rsdr>(BlockRng<ReseedingCore<R, Rsdr>>)
[00:05:47] 89 | | where R: BlockRngCore + SeedableRng,
[00:05:47] 90 | |       Rsdr: RngCore;
[00:05:47] 
[00:05:47] 
[00:05:47] error[E0277]: the trait bound `rand_hc::Hc128Core: rand_core::SeedableRng` is not satisfied
[00:05:47]   --> <::std::thread::local::__thread_local_inner macros>:5:14
[00:05:47] 1  |      (
[00:05:47]    |    __-
[00:05:47]    |   |__|
[00:05:47]    |  ||
[00:05:47]    |  ||
[00:05:47] 2  |  ||  @ key $ ( # [ $ attr : meta ] ) * $ vis : vis $ name : ident , $ t : ty , $
[00:05:47] 3  |  ||  init : expr ) => {
[00:05:47] 4  |  ||  {
[00:05:47] 5  |  ||  # [ inline ] fn __init (  ) -> $ t { $ init } unsafe fn __getit (  ) -> $
[00:05:47]    |  ||               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_hc::Hc128Core`
[00:05:47] ...   ||
[00:05:47] 28 |  ||  > = $ crate :: __thread_local_inner ! (
[00:05:47]    |  ||______-
[00:05:47] 29 | |||  @ key $ ( # [ $ attr ] ) * $ vis $ name , $ t , $ init ) ; }
[00:05:47]    | |||_________________________________________________________|___|
[00:05:47]    | |||_________________________________________________________|___|
[00:05:47]    | ||__________________________________________________________|___in this expansion of `$crate::__thread_local_inner!` (#2)
[00:05:47]    | |___________________________________________________________|   in this expansion of `$crate::__thread_local_inner!` (#3)
[00:05:47]    | 
[00:05:47]   ::: <::std::thread::local::thread_local macros>:1:1
[00:05:47]    |
[00:05:47] 1  |    / (  ) => {  } ; (
[00:05:47] 1  |    / (  ) => {  } ; (
[00:05:47] 2  |    | $ ( # [ $ attr : meta ] ) * $ vis : vis static $ name : ident : $ t : ty = $
[00:05:47] 3  |    | init : expr ; $ ( $ rest : tt ) * ) => (
[00:05:47] 4  |    | $ crate :: __thread_local_inner ! (
[00:05:47] ...     |
[00:05:47] 9  |    | $ crate :: __thread_local_inner ! (
[00:05:47]    |  __|_-
[00:05:47] 10 | |  | $ ( # [ $ attr ] ) * $ vis $ name , $ t , $ init ) ; ) ;
[00:05:47]    | |  |____________________________________________________-___- in this expansion of `thread_local!` (#1)
[00:05:47]    |                                                         in this macro invocation (#2)
[00:05:47]    | 
[00:05:47]    | 
[00:05:47]   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/thread.rs:76:1
[00:05:47] 76 | /    thread_local!(
[00:05:47] 76 | /    thread_local!(
[00:05:47] 77 | |        static THREAD_RNG_KEY: UnsafeCell<ReseedingRng<Hc128Core, EntropyRng>> = {
[00:05:47] 78 | |            let mut entropy_source = EntropyRng::new();
[00:05:47] 79 | |            let r = Hc128Core::from_rng(&mut entropy_source).unwrap_or_else(|err|
[00:05:47] 85 | |        }
[00:05:47] 86 | |    );
[00:05:47]    | |_____- in this macro invocation (#1)
[00:05:47]    |
[00:05:47]    |
[00:05:47] note: required by `rngs::adapter::reseeding::ReseedingRng`
[00:05:47]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/adapter/reseeding.rs:88:1
[00:05:47]    |
[00:05:47] 88 | / pub struct ReseedingRng<R, Rsdr>(BlockRng<ReseedingCore<R, Rsdr>>)
[00:05:47] 89 | | where R: BlockRngCore + SeedableRng,
[00:05:47] 90 | |       Rsdr: RngCore;
[00:05:47] 
[00:05:47] 
[00:05:47] error[E0277]: the trait bound `rand_hc::Hc128Core: rand_core::block::BlockRngCore` is not satisfied
[00:05:47]   ::: <::std::thread::local::thread_local macros>:1:1
[00:05:47]    |
[00:05:47] 1  |     / (  ) => {  } ; (
[00:05:47] 1  |     / (  ) => {  } ; (
[00:05:47] 2  |     | $ ( # [ $ attr : meta ] ) * $ vis : vis static $ name : ident : $ t : ty = $
[00:05:47] 3  |     | init : expr ; $ ( $ rest : tt ) * ) => (
[00:05:47] 4  |     | $ crate :: __thread_local_inner ! (
[00:05:47] ...      |
[00:05:47] 9  |     | $ crate :: __thread_local_inner ! (
[00:05:47]    |  ___|_-
[00:05:47] 10 | |   | $ ( # [ $ attr ] ) * $ vis $ name , $ t , $ init ) ; ) ;
[00:05:47]    | |   |____________________________________________________-___- in this expansion of `thread_local!` (#1)
[00:05:47]    |                                                          in this macro invocation (#2)
[00:05:47]    |                                                          in this macro invocation (#2)
[00:05:47]   --> <::std::thread::local::__thread_local_inner macros>:5:47
[00:05:47] 1  |       (
[00:05:47]    |     __-
[00:05:47]    |    |__|
[00:05:47]    |   ||
[00:05:47]    |   ||
[00:05:47] 2  |   ||  @ key $ ( # [ $ attr : meta ] ) * $ vis : vis $ name : ident , $ t : ty , $
[00:05:47] 3  |   ||  init : expr ) => {
[00:05:47] 4  |   ||  {
[00:05:47] 5  |   ||  # [ inline ] fn __init (  ) -> $ t { $ init } unsafe fn __getit (  ) -> $
[00:05:47]    |   ||________________________________________________^
[00:05:47] 6  |  |||  crate :: option :: Option < & 'static $ crate :: cell :: UnsafeCell < $ crate
[00:05:47] 7  |  |||  :: option :: Option < $ t >> > {
[00:05:47] 8  |  |||  # [
[00:05:47] ...   |||
[00:05:47] 22 |  |||  static __KEY : $ crate :: thread :: __OsLocalKeyInner < $ t > = $ crate ::
[00:05:47] 23 |  |||  thread :: __OsLocalKeyInner :: new (  ) ; __KEY . get (  ) } unsafe {
[00:05:47]    |  |||_____________________________________________________________^ the trait `rand_core::block::BlockRngCore` is not implemented for `rand_hc::Hc128Core`
[00:05:47] ...    ||
[00:05:47] 28 |   ||  > = $ crate :: __thread_local_inner ! (
[00:05:47]    |  _||______-
[00:05:47] 29 | | ||  @ key $ ( # [ $ attr ] ) * $ vis $ name , $ t , $ init ) ; }
[00:05:47]    | | ||_________________________________________________________|___|
[00:05:47]    | | ||_________________________________________________________|___|
[00:05:47]    | | |__________________________________________________________|___in this expansion of `$crate::__thread_local_inner!` (#2)
[00:05:47]    | |____________________________________________________________|   in this expansion of `$crate::__thread_local_inner!` (#3)
[00:05:47]    | 
[00:05:47]    | 
[00:05:47]   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/thread.rs:76:1
[00:05:47] 76 | /     thread_local!(
[00:05:47] 76 | /     thread_local!(
[00:05:47] 77 | |         static THREAD_RNG_KEY: UnsafeCell<ReseedingRng<Hc128Core, EntropyRng>> = {
[00:05:47] 78 | |             let mut entropy_source = EntropyRng::new();
[00:05:47] 79 | |             let r = Hc128Core::from_rng(&mut entropy_source).unwrap_or_else(|err|
[00:05:47] 85 | |         }
[00:05:47] 86 | |     );
[00:05:47]    | |______- in this macro invocation (#1)
[00:05:47]    |
[00:05:47]    |
[00:05:47] note: required by `rngs::adapter::reseeding::ReseedingRng`
[00:05:47]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/adapter/reseeding.rs:88:1
[00:05:47]    |
[00:05:47] 88 | / pub struct ReseedingRng<R, Rsdr>(BlockRng<ReseedingCore<R, Rsdr>>)
[00:05:47] 89 | | where R: BlockRngCore + SeedableRng,
[00:05:47] 90 | |       Rsdr: RngCore;
[00:05:47] 
[00:05:47] 
[00:05:47] error[E0277]: the trait bound `rand_hc::Hc128Core: rand_core::SeedableRng` is not satisfied
[00:05:47]   ::: <::std::thread::local::thread_local macros>:1:1
[00:05:47]    |
[00:05:47] 1  |     / (  ) => {  } ; (
[00:05:47] 1  |     / (  ) => {  } ; (
[00:05:47] 2  |     | $ ( # [ $ attr : meta ] ) * $ vis : vis static $ name : ident : $ t : ty = $
[00:05:47] 3  |     | init : expr ; $ ( $ rest : tt ) * ) => (
[00:05:47] 4  |     | $ crate :: __thread_local_inner ! (
[00:05:47] ...      |
[00:05:47] 9  |     | $ crate :: __thread_local_inner ! (
[00:05:47]    |  ___|_-
[00:05:47] 10 | |   | $ ( # [ $ attr ] ) * $ vis $ name , $ t , $ init ) ; ) ;
[00:05:47]    | |   |____________________________________________________-___- in this expansion of `thread_local!` (#1)
[00:05:47]    |                                                          in this macro invocation (#2)
[00:05:47]    |                                                          in this macro invocation (#2)
[00:05:47]   --> <::std::thread::local::__thread_local_inner macros>:5:47
[00:05:47] 1  |       (
[00:05:47]    |     __-
[00:05:47]    |    |__|
[00:05:47]    |   ||
[00:05:47]    |   ||
[00:05:47] 2  |   ||  @ key $ ( # [ $ attr : meta ] ) * $ vis : vis $ name : ident , $ t : ty , $
[00:05:47] 3  |   ||  init : expr ) => {
[00:05:47] 4  |   ||  {
[00:05:47] 5  |   ||  # [ inline ] fn __init (  ) -> $ t { $ init } unsafe fn __getit (  ) -> $
[00:05:47]    |   ||________________________________________________^
[00:05:47] 6  |  |||  crate :: option :: Option < & 'static $ crate :: cell :: UnsafeCell < $ crate
[00:05:47] 7  |  |||  :: option :: Option < $ t >> > {
[00:05:47] 8  |  |||  # [
[00:05:47] ...   |||
[00:05:47] 22 |  |||  static __KEY : $ crate :: thread :: __OsLocalKeyInner < $ t > = $ crate ::
[00:05:47] 23 |  |||  thread :: __OsLocalKeyInner :: new (  ) ; __KEY . get (  ) } unsafe {
[00:05:47]    |  |||_____________________________________________________________^ the trait `rand_core::SeedableRng` is not implemented for `rand_hc::Hc128Core`
[00:05:47] ...    ||
[00:05:47] 28 |   ||  > = $ crate :: __thread_local_inner ! (
[00:05:47]    |  _||______-
[00:05:47] 29 | | ||  @ key $ ( # [ $ attr ] ) * $ vis $ name , $ t , $ init ) ; }
[00:05:47]    | | ||_________________________________________________________|___|
[00:05:47]    | | ||_________________________________________________________|___|
[00:05:47]    | | |__________________________________________________________|___in this expansion of `$crate::__thread_local_inner!` (#2)
[00:05:47]    | |____________________________________________________________|   in this expansion of `$crate::__thread_local_inner!` (#3)
[00:05:47]    | 
[00:05:47]    | 
[00:05:47]   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/thread.rs:76:1
[00:05:47] 76 | /     thread_local!(
[00:05:47] 76 | /     thread_local!(
[00:05:47] 77 | |         static THREAD_RNG_KEY: UnsafeCell<ReseedingRng<Hc128Core, EntropyRng>> = {
[00:05:47] 78 | |             let mut entropy_source = EntropyRng::new();
[00:05:47] 79 | |             let r = Hc128Core::from_rng(&mut entropy_source).unwrap_or_else(|err|
[00:05:47] 85 | |         }
[00:05:47] 86 | |     );
[00:05:47]    | |______- in this macro invocation (#1)
[00:05:47]    |
[00:05:47]    |
[00:05:47] note: required by `rngs::adapter::reseeding::ReseedingRng`
[00:05:47]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/adapter/reseeding.rs:88:1
[00:05:47]    |
[00:05:47] 88 | / pub struct ReseedingRng<R, Rsdr>(BlockRng<ReseedingCore<R, Rsdr>>)
[00:05:47] 89 | | where R: BlockRngCore + SeedableRng,
[00:05:47] 90 | |       Rsdr: RngCore;
[00:05:47] 
[00:05:47] 
[00:05:47] error[E0277]: the trait bound `rand_hc::Hc128Core: rand_core::block::BlockRngCore` is not satisfied
[00:05:47]   --> <::std::thread::local::__thread_local_inner macros>:16:16
[00:05:47] 1  |      (
[00:05:47]    |    __-
[00:05:47]    |   |__|
[00:05:47]    |  ||
[00:05:47]    |  ||
[00:05:47] 2  |  ||  @ key $ ( # [ $ attr : meta ] ) * $ vis : vis $ name : ident , $ t : ty , $
[00:05:47] 3  |  ||  init : expr ) => {
[00:05:47] 4  |  ||  {
[00:05:47] ...   ||
[00:05:47] 16 |  ||  static __KEY : $ crate :: thread :: __FastLocalKeyInner < $ t > = $ crate ::
[00:05:47]    |  ||                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rand_core::block::BlockRngCore` is not implemented for `rand_hc::Hc128Core`
[00:05:47] ...   ||
[00:05:47] 28 |  ||  > = $ crate :: __thread_local_inner ! (
[00:05:47]    |  ||______-
[00:05:47] 29 | |||  @ key $ ( # [ $ attr ] ) * $ vis $ name , $ t , $ init ) ; }
[00:05:47]    | |||_________________________________________________________|___|
[00:05:47]    | |||_________________________________________________________|___|
[00:05:47]    | ||__________________________________________________________|___in this expansion of `$crate::__thread_local_inner!` (#2)
[00:05:47]    | |___________________________________________________________|   in this expansion of `$crate::__thread_local_inner!` (#3)
[00:05:47]    | 
[00:05:47]   ::: <::std::thread::local::thread_local macros>:1:1
[00:05:47]    |
[00:05:47] 1  |    / (  ) => {  } ; (
[00:05:47] 1  |    / (  ) => {  } ; (
[00:05:47] 2  |    | $ ( # [ $ attr : meta ] ) * $ vis : vis static $ name : ident : $ t : ty = $
[00:05:47] 3  |    | init : expr ; $ ( $ rest : tt ) * ) => (
[00:05:47] 4  |    | $ crate :: __thread_local_inner ! (
[00:05:47] ...     |
[00:05:47] 9  |    | $ crate :: __thread_local_inner ! (
[00:05:47]    |  __|_-
[00:05:47] 10 | |  | $ ( # [ $ attr ] ) * $ vis $ name , $ t , $ init ) ; ) ;
[00:05:47]    | |  |____________________________________________________-___- in this expansion of `thread_local!` (#1)
[00:05:47]    |                                                         in this macro invocation (#2)
[00:05:47]    | 
[00:05:47]    | 
[00:05:47]   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/thread.rs:76:1
[00:05:47] 76 | /    thread_local!(
[00:05:47] 76 | /    thread_local!(
[00:05:47] 77 | |        static THREAD_RNG_KEY: UnsafeCell<ReseedingRng<Hc128Core, EntropyRng>> = {
[00:05:47] 78 | |            let mut entropy_source = EntropyRng::new();
[00:05:47] 79 | |            let r = Hc128Core::from_rng(&mut entropy_source).unwrap_or_else(|err|
[00:05:47] 85 | |        }
[00:05:47] 86 | |    );
[00:05:47]    | |_____- in this macro invocation (#1)
[00:05:47]    |
[00:05:47]    |
[00:05:47] note: required by `rngs::adapter::reseeding::ReseedingRng`
[00:05:47]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/adapter/reseeding.rs:88:1
[00:05:47]    |
[00:05:47] 88 | / pub struct ReseedingRng<R, Rsdr>(BlockRng<ReseedingCore<R, Rsdr>>)
[00:05:47] 89 | | where R: BlockRngCore + SeedableRng,
[00:05:47] 90 | |       Rsdr: RngCore;
[00:05:47] 
[00:05:47] 
[00:05:47] error[E0277]: the trait bound `rand_hc::Hc128Core: rand_core::SeedableRng` is not satisfied
[00:05:47]   --> <::std::thread::local::__thread_local_inner macros>:16:16
[00:05:47] 1  |      (
[00:05:47]    |    __-
[00:05:47]    |   |__|
[00:05:47]    |  ||
[00:05:47]    |  ||
[00:05:47] 2  |  ||  @ key $ ( # [ $ attr : meta ] ) * $ vis : vis $ name : ident , $ t : ty , $
[00:05:47] 3  |  ||  init : expr ) => {
[00:05:47] 4  |  ||  {
[00:05:47] ...   ||
[00:05:47] 16 |  ||  static __KEY : $ crate :: thread :: __FastLocalKeyInner < $ t > = $ crate ::
[00:05:47]    |  ||                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_hc::Hc128Core`
[00:05:47] ...   ||
[00:05:47] 28 |  ||  > = $ crate :: __thread_local_inner ! (
[00:05:47]    |  ||______-
[00:05:47] 29 | |||  @ key $ ( # [ $ attr ] ) * $ vis $ name , $ t , $ init ) ; }
[00:05:47]    | |||_________________________________________________________|___|
[00:05:47]    | |||_________________________________________________________|___|
[00:05:47]    | ||__________________________________________________________|___in this expansion of `$crate::__thread_local_inner!` (#2)
[00:05:47]    | |___________________________________________________________|   in this expansion of `$crate::__thread_local_inner!` (#3)
[00:05:47]    | 
[00:05:47]   ::: <::std::thread::local::thread_local macros>:1:1
[00:05:47]    |
[00:05:47] 1  |    / (  ) => {  } ; (
[00:05:47] 1  |    / (  ) => {  } ; (
[00:05:47] 2  |    | $ ( # [ $ attr : meta ] ) * $ vis : vis static $ name : ident : $ t : ty = $
[00:05:47] 3  |    | init : expr ; $ ( $ rest : tt ) * ) => (
[00:05:47] 4  |    | $ crate :: __thread_local_inner ! (
[00:05:47] ...     |
[00:05:47] 9  |    | $ crate :: __thread_local_inner ! (
[00:05:47]    |  __|_-
[00:05:47] 10 | |  | $ ( # [ $ attr ] ) * $ vis $ name , $ t , $ init ) ; ) ;
[00:05:47]    | |  |____________________________________________________-___- in this expansion of `thread_local!` (#1)
---
travis_time:end:24feb1f7:start=1555081556753908476,finish=1555081556758306487,duration=4398011
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:30195a4d
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2f6e708b
travis_time:start:2f6e708b
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0163aa6c
$ dmesg | grep -i kill
