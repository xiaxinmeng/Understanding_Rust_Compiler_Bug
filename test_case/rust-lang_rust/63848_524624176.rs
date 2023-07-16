
error[E0277]: the trait bound `rand_pcg::Mcg128Xsl64: rand_core::SeedableRng` is not satisfied
  --> /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/small.rs:96:6
   |
96 | impl SeedableRng for SmallRng {
   |      ^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_pcg::Mcg128Xsl64`

error[E0277]: the trait bound `rand_hc::Hc128Rng: rand_core::SeedableRng` is not satisfied
  --> /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/std.rs:55:6
   |
55 | impl SeedableRng for StdRng {
   |      ^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_hc::Hc128Rng`

error[E0277]: the trait bound `rand_isaac::IsaacRng: rand_core::SeedableRng` is not satisfied
  --> /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/deprecated.rs:50:6
   |
50 | impl SeedableRng for IsaacRng {
   |      ^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_isaac::IsaacRng`

error[E0277]: the trait bound `rand_isaac::Isaac64Rng: rand_core::SeedableRng` is not satisfied
  --> /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/deprecated.rs:96:6
   |
96 | impl SeedableRng for Isaac64Rng {
   |      ^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_isaac::Isaac64Rng`

error[E0277]: the trait bound `rand_chacha::ChaChaRng: rand_core::SeedableRng` is not satisfied
   --> /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/deprecated.rs:141:6
    |
141 | impl SeedableRng for ChaChaRng {
    |      ^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_chacha::ChaChaRng`

error[E0277]: the trait bound `rand_hc::Hc128Rng: rand_core::SeedableRng` is not satisfied
   --> /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/deprecated.rs:198:6
    |
198 | impl SeedableRng for Hc128Rng {
    |      ^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_hc::Hc128Rng`

error[E0277]: the trait bound `rand_xorshift::XorShiftRng: rand_core::SeedableRng` is not satisfied
   --> /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/deprecated.rs:239:6
    |
239 | impl SeedableRng for XorShiftRng {
    |      ^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_xorshift::XorShiftRng`

error[E0277]: the trait bound `rand_hc::Hc128Rng: rand_core::SeedableRng` is not satisfied
   --> /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/deprecated.rs:279:6
    |
279 | impl SeedableRng for StdRng {
    |      ^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_hc::Hc128Rng`

error[E0277]: the trait bound `rand_hc::Hc128Core: rand_core::block::BlockRngCore` is not satisfied
  --> /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/thread.rs:73:5
   |
73 |     rng: *mut ReseedingRng<Hc128Core, EntropyRng>,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rand_core::block::BlockRngCore` is not implemented for `rand_hc::Hc128Core`
   |
note: required by `rngs::adapter::reseeding::ReseedingRng`
  --> /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/adapter/reseeding.rs:88:1
   |
88 | / pub struct ReseedingRng<R, Rsdr>(BlockRng<ReseedingCore<R, Rsdr>>)
89 | | where R: BlockRngCore + SeedableRng,
90 | |       Rsdr: RngCore;
   | |____________________^

error[E0277]: the trait bound `rand_hc::Hc128Core: rand_core::SeedableRng` is not satisfied
  --> /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/thread.rs:73:5
   |
73 |     rng: *mut ReseedingRng<Hc128Core, EntropyRng>,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_hc::Hc128Core`
   |
note: required by `rngs::adapter::reseeding::ReseedingRng`
  --> /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/adapter/reseeding.rs:88:1
   |
88 | / pub struct ReseedingRng<R, Rsdr>(BlockRng<ReseedingCore<R, Rsdr>>)
89 | | where R: BlockRngCore + SeedableRng,
90 | |       Rsdr: RngCore;
   | |____________________^

error[E0277]: the trait bound `rand_hc::Hc128Core: rand_core::block::BlockRngCore` is not satisfied
   | 
  ::: <::std::thread::local::thread_local macros>:1:1
   |
1  |   / () => { } ;
2  |   | ($ (# [$ attr : meta]) * $ vis : vis static $ name : ident : $ t : ty = $ init
3  |   |  : expr ; $ ($ rest : tt) *) =>
4  |   | ($ crate :: __thread_local_inner !
...    |
9  |   | ($ crate :: __thread_local_inner !
   |  _|__-
10 | | |  ($ (# [$ attr]) * $ vis $ name , $ t , $ init) ;) ;
   | | |_________________________________________________-__- in this expansion of `thread_local!`
   | |___________________________________________________|
   |                                                     in this macro invocation
  --> <::std::thread::local::__thread_local_inner macros>:36:43
   |
1  |  /  (@ key $ (# [$ attr : meta]) * $ vis : vis $ name : ident , $ t : ty , $ init
2  |  |   : expr) =>
3  |  |  {
4  |  |      {
...   |
36 |  |      $ (# [$ attr]) * $ vis const $ name : $ crate :: thread :: LocalKey < $ t
   |  |____________________________________________^
37 | ||      > = $ crate :: __thread_local_inner !
   | ||______^ the trait `rand_core::block::BlockRngCore` is not implemented for `rand_hc::Hc128Core`
38 |  |      (@ key $ (# [$ attr]) * $ vis $ name , $ t , $ init) ;
39 |  |  }
   |  |__- in this expansion of `$crate::__thread_local_inner!`
   | 
  ::: /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/thread.rs:76:1
   |
76 | /   thread_local!(
77 | |       static THREAD_RNG_KEY: UnsafeCell<ReseedingRng<Hc128Core, EntropyRng>> = {
78 | |           let mut entropy_source = EntropyRng::new();
79 | |           let r = Hc128Core::from_rng(&mut entropy_source).unwrap_or_else(|err|
...  |
85 | |       }
86 | |   );
   | |____- in this macro invocation
   |
note: required by `rngs::adapter::reseeding::ReseedingRng`
  --> /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/adapter/reseeding.rs:88:1
   |
88 | / pub struct ReseedingRng<R, Rsdr>(BlockRng<ReseedingCore<R, Rsdr>>)
89 | | where R: BlockRngCore + SeedableRng,
90 | |       Rsdr: RngCore;
   | |____________________^

error[E0277]: the trait bound `rand_hc::Hc128Core: rand_core::SeedableRng` is not satisfied
   | 
  ::: <::std::thread::local::thread_local macros>:1:1
   |
1  |   / () => { } ;
2  |   | ($ (# [$ attr : meta]) * $ vis : vis static $ name : ident : $ t : ty = $ init
3  |   |  : expr ; $ ($ rest : tt) *) =>
4  |   | ($ crate :: __thread_local_inner !
...    |
9  |   | ($ crate :: __thread_local_inner !
   |  _|__-
10 | | |  ($ (# [$ attr]) * $ vis $ name , $ t , $ init) ;) ;
   | | |_________________________________________________-__- in this expansion of `thread_local!`
   | |___________________________________________________|
   |                                                     in this macro invocation
  --> <::std::thread::local::__thread_local_inner macros>:36:43
   |
1  |  /  (@ key $ (# [$ attr : meta]) * $ vis : vis $ name : ident , $ t : ty , $ init
2  |  |   : expr) =>
3  |  |  {
4  |  |      {
...   |
36 |  |      $ (# [$ attr]) * $ vis const $ name : $ crate :: thread :: LocalKey < $ t
   |  |____________________________________________^
37 | ||      > = $ crate :: __thread_local_inner !
   | ||______^ the trait `rand_core::SeedableRng` is not implemented for `rand_hc::Hc128Core`
38 |  |      (@ key $ (# [$ attr]) * $ vis $ name , $ t , $ init) ;
39 |  |  }
   |  |__- in this expansion of `$crate::__thread_local_inner!`
   | 
  ::: /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/thread.rs:76:1
   |
76 | /   thread_local!(
77 | |       static THREAD_RNG_KEY: UnsafeCell<ReseedingRng<Hc128Core, EntropyRng>> = {
78 | |           let mut entropy_source = EntropyRng::new();
79 | |           let r = Hc128Core::from_rng(&mut entropy_source).unwrap_or_else(|err|
...  |
85 | |       }
86 | |   );
   | |____- in this macro invocation
   |
note: required by `rngs::adapter::reseeding::ReseedingRng`
  --> /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/adapter/reseeding.rs:88:1
   |
88 | / pub struct ReseedingRng<R, Rsdr>(BlockRng<ReseedingCore<R, Rsdr>>)
89 | | where R: BlockRngCore + SeedableRng,
90 | |       Rsdr: RngCore;
   | |____________________^

error[E0277]: the trait bound `rand_hc::Hc128Core: rand_core::block::BlockRngCore` is not satisfied
  --> <::std::thread::local::__thread_local_inner macros>:5:20
   |
1  |  /   (@ key $ (# [$ attr : meta]) * $ vis : vis $ name : ident , $ t : ty , $ init
2  |  |    : expr) =>
3  |  |   {
4  |  |       {
5  |  |           # [inline] fn __init () -> $ t { $ init } unsafe fn __getit () -> $
   |  |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rand_core::block::BlockRngCore` is not implemented for `rand_hc::Hc128Core`
...   |
37 |  |       > = $ crate :: __thread_local_inner !
   |  |___________-
38 | ||       (@ key $ (# [$ attr]) * $ vis $ name , $ t , $ init) ;
   | ||__________________________________________________________- in this macro invocation (#3)
39 |  |   }
   |  |   -
   |  |   |
   |  |___in this expansion of `$crate::__thread_local_inner!` (#2)
   |      in this expansion of `$crate::__thread_local_inner!` (#3)
   | 
  ::: <::std::thread::local::thread_local macros>:1:1
   |
1  |    / () => { } ;
2  |    | ($ (# [$ attr : meta]) * $ vis : vis static $ name : ident : $ t : ty = $ init
3  |    |  : expr ; $ ($ rest : tt) *) =>
4  |    | ($ crate :: __thread_local_inner !
...     |
9  |    | ($ crate :: __thread_local_inner !
   |  __|__-
10 | |  |  ($ (# [$ attr]) * $ vis $ name , $ t , $ init) ;) ;
   | |  |_________________________________________________-__- in this expansion of `thread_local!` (#1)
   | |____________________________________________________|
   |                                                      in this macro invocation (#2)
   | 
  ::: /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/thread.rs:76:1
   |
76 | /    thread_local!(
77 | |        static THREAD_RNG_KEY: UnsafeCell<ReseedingRng<Hc128Core, EntropyRng>> = {
78 | |            let mut entropy_source = EntropyRng::new();
79 | |            let r = Hc128Core::from_rng(&mut entropy_source).unwrap_or_else(|err|
...  |
85 | |        }
86 | |    );
   | |_____- in this macro invocation (#1)
   |
note: required by `rngs::adapter::reseeding::ReseedingRng`
  --> /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/adapter/reseeding.rs:88:1
   |
88 | / pub struct ReseedingRng<R, Rsdr>(BlockRng<ReseedingCore<R, Rsdr>>)
89 | | where R: BlockRngCore + SeedableRng,
90 | |       Rsdr: RngCore;
   | |____________________^

error[E0277]: the trait bound `rand_hc::Hc128Core: rand_core::SeedableRng` is not satisfied
  --> <::std::thread::local::__thread_local_inner macros>:5:20
   |
1  |  /   (@ key $ (# [$ attr : meta]) * $ vis : vis $ name : ident , $ t : ty , $ init
2  |  |    : expr) =>
3  |  |   {
4  |  |       {
5  |  |           # [inline] fn __init () -> $ t { $ init } unsafe fn __getit () -> $
   |  |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_hc::Hc128Core`
...   |
37 |  |       > = $ crate :: __thread_local_inner !
   |  |___________-
38 | ||       (@ key $ (# [$ attr]) * $ vis $ name , $ t , $ init) ;
   | ||__________________________________________________________- in this macro invocation (#3)
39 |  |   }
   |  |   -
   |  |   |
   |  |___in this expansion of `$crate::__thread_local_inner!` (#2)
   |      in this expansion of `$crate::__thread_local_inner!` (#3)
   | 
  ::: <::std::thread::local::thread_local macros>:1:1
   |
1  |    / () => { } ;
2  |    | ($ (# [$ attr : meta]) * $ vis : vis static $ name : ident : $ t : ty = $ init
3  |    |  : expr ; $ ($ rest : tt) *) =>
4  |    | ($ crate :: __thread_local_inner !
...     |
9  |    | ($ crate :: __thread_local_inner !
   |  __|__-
10 | |  |  ($ (# [$ attr]) * $ vis $ name , $ t , $ init) ;) ;
   | |  |_________________________________________________-__- in this expansion of `thread_local!` (#1)
   | |____________________________________________________|
   |                                                      in this macro invocation (#2)
   | 
  ::: /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/thread.rs:76:1
   |
76 | /    thread_local!(
77 | |        static THREAD_RNG_KEY: UnsafeCell<ReseedingRng<Hc128Core, EntropyRng>> = {
78 | |            let mut entropy_source = EntropyRng::new();
79 | |            let r = Hc128Core::from_rng(&mut entropy_source).unwrap_or_else(|err|
...  |
85 | |        }
86 | |    );
   | |_____- in this macro invocation (#1)
   |
note: required by `rngs::adapter::reseeding::ReseedingRng`
  --> /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/adapter/reseeding.rs:88:1
   |
88 | / pub struct ReseedingRng<R, Rsdr>(BlockRng<ReseedingCore<R, Rsdr>>)
89 | | where R: BlockRngCore + SeedableRng,
90 | |       Rsdr: RngCore;
   | |____________________^

error[E0277]: the trait bound `rand_hc::Hc128Core: rand_core::block::BlockRngCore` is not satisfied
   | 
  ::: <::std::thread::local::thread_local macros>:1:1
   |
1  |     / () => { } ;
2  |     | ($ (# [$ attr : meta]) * $ vis : vis static $ name : ident : $ t : ty = $ init
3  |     |  : expr ; $ ($ rest : tt) *) =>
4  |     | ($ crate :: __thread_local_inner !
...      |
9  |     | ($ crate :: __thread_local_inner !
   |  ___|__-
10 | |   |  ($ (# [$ attr]) * $ vis $ name , $ t , $ init) ;) ;
   | |   |_________________________________________________-__- in this expansion of `thread_local!` (#1)
   | |_____________________________________________________|
   |                                                       in this macro invocation (#2)
  --> <::std::thread::local::__thread_local_inner macros>:5:51
   |
1  |   /   (@ key $ (# [$ attr : meta]) * $ vis : vis $ name : ident , $ t : ty , $ init
2  |   |    : expr) =>
3  |   |   {
4  |   |       {
5  |   |           # [inline] fn __init () -> $ t { $ init } unsafe fn __getit () -> $
   |   |_____________________________________________________^
6  |  ||           crate :: option :: Option < & 'static $ t >
7  |  ||           {
8  |  ||               #
...   ||
29 |  ||               get (__init)
30 |  ||           } unsafe { $ crate :: thread :: LocalKey :: new (__getit) }
   |  ||___________^ the trait `rand_core::block::BlockRngCore` is not implemented for `rand_hc::Hc128Core`
...    |
37 |   |       > = $ crate :: __thread_local_inner !
   |  _|___________-
38 | | |       (@ key $ (# [$ attr]) * $ vis $ name , $ t , $ init) ;
   | |_|__________________________________________________________- in this macro invocation (#3)
39 |   |   }
   |   |   -
   |   |   |
   |   |___in this expansion of `$crate::__thread_local_inner!` (#2)
   |       in this expansion of `$crate::__thread_local_inner!` (#3)
   | 
  ::: /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/thread.rs:76:1
   |
76 | /     thread_local!(
77 | |         static THREAD_RNG_KEY: UnsafeCell<ReseedingRng<Hc128Core, EntropyRng>> = {
78 | |             let mut entropy_source = EntropyRng::new();
79 | |             let r = Hc128Core::from_rng(&mut entropy_source).unwrap_or_else(|err|
...  |
85 | |         }
86 | |     );
   | |______- in this macro invocation (#1)
   |
note: required by `rngs::adapter::reseeding::ReseedingRng`
  --> /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/adapter/reseeding.rs:88:1
   |
88 | / pub struct ReseedingRng<R, Rsdr>(BlockRng<ReseedingCore<R, Rsdr>>)
89 | | where R: BlockRngCore + SeedableRng,
90 | |       Rsdr: RngCore;
   | |____________________^

error[E0277]: the trait bound `rand_hc::Hc128Core: rand_core::SeedableRng` is not satisfied
   | 
  ::: <::std::thread::local::thread_local macros>:1:1
   |
1  |     / () => { } ;
2  |     | ($ (# [$ attr : meta]) * $ vis : vis static $ name : ident : $ t : ty = $ init
3  |     |  : expr ; $ ($ rest : tt) *) =>
4  |     | ($ crate :: __thread_local_inner !
...      |
9  |     | ($ crate :: __thread_local_inner !
   |  ___|__-
10 | |   |  ($ (# [$ attr]) * $ vis $ name , $ t , $ init) ;) ;
   | |   |_________________________________________________-__- in this expansion of `thread_local!` (#1)
   | |_____________________________________________________|
   |                                                       in this macro invocation (#2)
  --> <::std::thread::local::__thread_local_inner macros>:5:51
   |
1  |   /   (@ key $ (# [$ attr : meta]) * $ vis : vis $ name : ident , $ t : ty , $ init
2  |   |    : expr) =>
3  |   |   {
4  |   |       {
5  |   |           # [inline] fn __init () -> $ t { $ init } unsafe fn __getit () -> $
   |   |_____________________________________________________^
6  |  ||           crate :: option :: Option < & 'static $ t >
7  |  ||           {
8  |  ||               #
...   ||
29 |  ||               get (__init)
30 |  ||           } unsafe { $ crate :: thread :: LocalKey :: new (__getit) }
   |  ||___________^ the trait `rand_core::SeedableRng` is not implemented for `rand_hc::Hc128Core`
...    |
37 |   |       > = $ crate :: __thread_local_inner !
   |  _|___________-
38 | | |       (@ key $ (# [$ attr]) * $ vis $ name , $ t , $ init) ;
   | |_|__________________________________________________________- in this macro invocation (#3)
39 |   |   }
   |   |   -
   |   |   |
   |   |___in this expansion of `$crate::__thread_local_inner!` (#2)
   |       in this expansion of `$crate::__thread_local_inner!` (#3)
   | 
  ::: /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/thread.rs:76:1
   |
76 | /     thread_local!(
77 | |         static THREAD_RNG_KEY: UnsafeCell<ReseedingRng<Hc128Core, EntropyRng>> = {
78 | |             let mut entropy_source = EntropyRng::new();
79 | |             let r = Hc128Core::from_rng(&mut entropy_source).unwrap_or_else(|err|
...  |
85 | |         }
86 | |     );
   | |______- in this macro invocation (#1)
   |
note: required by `rngs::adapter::reseeding::ReseedingRng`
  --> /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/adapter/reseeding.rs:88:1
   |
88 | / pub struct ReseedingRng<R, Rsdr>(BlockRng<ReseedingCore<R, Rsdr>>)
89 | | where R: BlockRngCore + SeedableRng,
90 | |       Rsdr: RngCore;
   | |____________________^

error[E0277]: the trait bound `rand_hc::Hc128Core: rand_core::block::BlockRngCore` is not satisfied
   | 
  ::: <::std::thread::local::thread_local macros>:1:1
   |
1  |    / () => { } ;
2  |    | ($ (# [$ attr : meta]) * $ vis : vis static $ name : ident : $ t : ty = $ init
3  |    |  : expr ; $ ($ rest : tt) *) =>
4  |    | ($ crate :: __thread_local_inner !
...     |
9  |    | ($ crate :: __thread_local_inner !
   |  __|__-
10 | |  |  ($ (# [$ attr]) * $ vis $ name , $ t , $ init) ;) ;
   | |  |_________________________________________________-__- in this expansion of `thread_local!` (#1)
   | |____________________________________________________|
   |                                                      in this macro invocation (#2)
  --> <::std::thread::local::__thread_local_inner macros>:20:36
   |
1  |  /   (@ key $ (# [$ attr : meta]) * $ vis : vis $ name : ident , $ t : ty , $ init
2  |  |    : expr) =>
3  |  |   {
4  |  |       {
...   |
20 |  |                  ,))] static __KEY : $ crate :: thread :: __FastLocalKeyInner <
   |  |______________________________________^
21 | ||               $ t > = $ crate :: thread :: __FastLocalKeyInner :: new () ; #
   | ||___________________^ the trait `rand_core::block::BlockRngCore` is not implemented for `rand_hc::Hc128Core`
...   |
37 |  |       > = $ crate :: __thread_local_inner !
   |  |___________-
38 | ||       (@ key $ (# [$ attr]) * $ vis $ name , $ t , $ init) ;
   | ||__________________________________________________________- in this macro invocation (#3)
39 |  |   }
   |  |   -
   |  |   |
   |  |___in this expansion of `$crate::__thread_local_inner!` (#2)
   |      in this expansion of `$crate::__thread_local_inner!` (#3)
   | 
  ::: /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/thread.rs:76:1
   |
76 | /    thread_local!(
77 | |        static THREAD_RNG_KEY: UnsafeCell<ReseedingRng<Hc128Core, EntropyRng>> = {
78 | |            let mut entropy_source = EntropyRng::new();
79 | |            let r = Hc128Core::from_rng(&mut entropy_source).unwrap_or_else(|err|
...  |
85 | |        }
86 | |    );
   | |_____- in this macro invocation (#1)
   |
note: required by `rngs::adapter::reseeding::ReseedingRng`
  --> /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/adapter/reseeding.rs:88:1
   |
88 | / pub struct ReseedingRng<R, Rsdr>(BlockRng<ReseedingCore<R, Rsdr>>)
89 | | where R: BlockRngCore + SeedableRng,
90 | |       Rsdr: RngCore;
   | |____________________^

error[E0277]: the trait bound `rand_hc::Hc128Core: rand_core::SeedableRng` is not satisfied
   | 
  ::: <::std::thread::local::thread_local macros>:1:1
   |
1  |    / () => { } ;
2  |    | ($ (# [$ attr : meta]) * $ vis : vis static $ name : ident : $ t : ty = $ init
3  |    |  : expr ; $ ($ rest : tt) *) =>
4  |    | ($ crate :: __thread_local_inner !
...     |
9  |    | ($ crate :: __thread_local_inner !
   |  __|__-
10 | |  |  ($ (# [$ attr]) * $ vis $ name , $ t , $ init) ;) ;
   | |  |_________________________________________________-__- in this expansion of `thread_local!` (#1)
   | |____________________________________________________|
   |                                                      in this macro invocation (#2)
  --> <::std::thread::local::__thread_local_inner macros>:20:36
   |
1  |  /   (@ key $ (# [$ attr : meta]) * $ vis : vis $ name : ident , $ t : ty , $ init
2  |  |    : expr) =>
3  |  |   {
4  |  |       {
...   |
20 |  |                  ,))] static __KEY : $ crate :: thread :: __FastLocalKeyInner <
   |  |______________________________________^
21 | ||               $ t > = $ crate :: thread :: __FastLocalKeyInner :: new () ; #
   | ||___________________^ the trait `rand_core::SeedableRng` is not implemented for `rand_hc::Hc128Core`
...   |
37 |  |       > = $ crate :: __thread_local_inner !
   |  |___________-
38 | ||       (@ key $ (# [$ attr]) * $ vis $ name , $ t , $ init) ;
   | ||__________________________________________________________- in this macro invocation (#3)
39 |  |   }
   |  |   -
   |  |   |
   |  |___in this expansion of `$crate::__thread_local_inner!` (#2)
   |      in this expansion of `$crate::__thread_local_inner!` (#3)
   | 
  ::: /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/thread.rs:76:1
   |
76 | /    thread_local!(
77 | |        static THREAD_RNG_KEY: UnsafeCell<ReseedingRng<Hc128Core, EntropyRng>> = {
78 | |            let mut entropy_source = EntropyRng::new();
79 | |            let r = Hc128Core::from_rng(&mut entropy_source).unwrap_or_else(|err|
...  |
85 | |        }
86 | |    );
   | |_____- in this macro invocation (#1)
   |
note: required by `rngs::adapter::reseeding::ReseedingRng`
  --> /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/adapter/reseeding.rs:88:1
   |
88 | / pub struct ReseedingRng<R, Rsdr>(BlockRng<ReseedingCore<R, Rsdr>>)
89 | | where R: BlockRngCore + SeedableRng,
90 | |       Rsdr: RngCore;
   | |____________________^

error[E0277]: the trait bound `rand_pcg::Mcg128Xsl64: rand_core::SeedableRng` is not satisfied
  --> /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/small.rs:97:5
   |
97 |     type Seed = <Rng as SeedableRng>::Seed;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_pcg::Mcg128Xsl64`

error[E0277]: the trait bound `rand_pcg::Mcg128Xsl64: rand_core::SeedableRng` is not satisfied
   --> /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/small.rs:99:5
    |
99  | /     fn from_seed(seed: Self::Seed) -> Self {
100 | |         SmallRng(Rng::from_seed(seed))
101 | |     }
    | |_____^ the trait `rand_core::SeedableRng` is not implemented for `rand_pcg::Mcg128Xsl64`

error[E0277]: the trait bound `rand_hc::Hc128Rng: rand_core::SeedableRng` is not satisfied
  --> /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/std.rs:56:5
   |
56 |     type Seed = <Hc128Rng as SeedableRng>::Seed;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_hc::Hc128Rng`

error[E0277]: the trait bound `rand_hc::Hc128Rng: rand_core::SeedableRng` is not satisfied
  --> /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/rngs/std.rs:58:5
   |
58 | /     fn from_seed(seed: Self::Seed) -> Self {
59 | |         StdRng(Hc128Rng::from_seed(seed))
60 | |     }
   | |_____^ the trait `rand_core::SeedableRng` is not implemented for `rand_hc::Hc128Rng`

error[E0277]: the trait bound `rand_isaac::IsaacRng: rand_core::SeedableRng` is not satisfied
  --> /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/deprecated.rs:51:5
   |
51 |     type Seed = <rand_isaac::IsaacRng as SeedableRng>::Seed;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_isaac::IsaacRng`

error[E0277]: the trait bound `rand_isaac::IsaacRng: rand_core::SeedableRng` is not satisfied
  --> /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/deprecated.rs:53:5
   |
53 | /     fn from_seed(seed: Self::Seed) -> Self {
54 | |         IsaacRng(rand_isaac::IsaacRng::from_seed(seed))
55 | |     }
   | |_____^ the trait `rand_core::SeedableRng` is not implemented for `rand_isaac::IsaacRng`

error[E0277]: the trait bound `rand_isaac::Isaac64Rng: rand_core::SeedableRng` is not satisfied
  --> /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/deprecated.rs:97:5
   |
97 |     type Seed = <rand_isaac::Isaac64Rng as SeedableRng>::Seed;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_isaac::Isaac64Rng`

error[E0277]: the trait bound `rand_isaac::Isaac64Rng: rand_core::SeedableRng` is not satisfied
   --> /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/deprecated.rs:99:5
    |
99  | /     fn from_seed(seed: Self::Seed) -> Self {
100 | |         Isaac64Rng(rand_isaac::Isaac64Rng::from_seed(seed))
101 | |     }
    | |_____^ the trait `rand_core::SeedableRng` is not implemented for `rand_isaac::Isaac64Rng`

error[E0277]: the trait bound `rand_chacha::ChaChaRng: rand_core::SeedableRng` is not satisfied
   --> /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/deprecated.rs:142:5
    |
142 |     type Seed = <rand_chacha::ChaChaRng as SeedableRng>::Seed;
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_chacha::ChaChaRng`

error[E0277]: the trait bound `rand_chacha::ChaChaRng: rand_core::SeedableRng` is not satisfied
   --> /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/deprecated.rs:144:5
    |
144 | /     fn from_seed(seed: Self::Seed) -> Self {
145 | |         ChaChaRng(rand_chacha::ChaChaRng::from_seed(seed))
146 | |     }
    | |_____^ the trait `rand_core::SeedableRng` is not implemented for `rand_chacha::ChaChaRng`

error[E0277]: the trait bound `rand_hc::Hc128Rng: rand_core::SeedableRng` is not satisfied
   --> /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/deprecated.rs:199:5
    |
199 |     type Seed = <rand_hc::Hc128Rng as SeedableRng>::Seed;
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_hc::Hc128Rng`

error[E0277]: the trait bound `rand_hc::Hc128Rng: rand_core::SeedableRng` is not satisfied
   --> /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/deprecated.rs:201:5
    |
201 | /     fn from_seed(seed: Self::Seed) -> Self {
202 | |         Hc128Rng(rand_hc::Hc128Rng::from_seed(seed))
203 | |     }
    | |_____^ the trait `rand_core::SeedableRng` is not implemented for `rand_hc::Hc128Rng`

error[E0277]: the trait bound `rand_xorshift::XorShiftRng: rand_core::SeedableRng` is not satisfied
   --> /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/deprecated.rs:240:5
    |
240 |     type Seed = <::rand_xorshift::XorShiftRng as SeedableRng>::Seed;
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_xorshift::XorShiftRng`

error[E0277]: the trait bound `rand_xorshift::XorShiftRng: rand_core::SeedableRng` is not satisfied
   --> /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/deprecated.rs:242:5
    |
242 | /     fn from_seed(seed: Self::Seed) -> Self {
243 | |         XorShiftRng(::rand_xorshift::XorShiftRng::from_seed(seed))
244 | |     }
    | |_____^ the trait `rand_core::SeedableRng` is not implemented for `rand_xorshift::XorShiftRng`

error[E0277]: the trait bound `rand_hc::Hc128Rng: rand_core::SeedableRng` is not satisfied
   --> /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/deprecated.rs:280:5
    |
280 |     type Seed = <rngs::StdRng as SeedableRng>::Seed;
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rand_core::SeedableRng` is not implemented for `rand_hc::Hc128Rng`

error[E0277]: the trait bound `rand_hc::Hc128Rng: rand_core::SeedableRng` is not satisfied
   --> /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.6.5/src/deprecated.rs:282:5
    |
282 | /     fn from_seed(seed: Self::Seed) -> Self {
283 | |         StdRng(rngs::StdRng::from_seed(seed))
284 | |     }
    | |_____^ the trait `rand_core::SeedableRng` is not implemented for `rand_hc::Hc128Rng`

error: aborting due to 34 previous errors
