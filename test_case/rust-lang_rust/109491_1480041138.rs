plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:24cb9080177205b6e8c946b17badbe402adc938f)
Download action repository 'rust-lang/simpleinfra@master' (SHA:5f3e9487b084c5235556ffd8baa8b183de9eb120)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-16core-64gb)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
   Compiling memmap2 v0.2.1
   Compiling perf-event-open-sys v1.0.1
   Compiling tempfile v3.3.0
   Compiling jobserver v0.1.26
error[E0275]: overflow evaluating the requirement `UInt<UInt<UInt<UInt<UInt<UInt<UInt<..., ...>, ...>, ...>, ...>, ...>, ...>, ...>: marker_traits::Unsigned`
     = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`typenum`)
     = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`typenum`)
note: required for `UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<..., ...>, ...>, ...>, ...>, ...>, ...>, ...>, ...>, ...>` to implement `Shl<B0>`
    --> /cargo/registry/src/index.crates.io-6f17d22bba15001f/typenum-1.16.0/src/uint.rs:874:27
     |
874  | impl<U: Unsigned, B: Bit> Shl<B0> for UInt<U, B> {
     |         |
     |         unsatisfied trait bound introduced here
     |         unsatisfied trait bound introduced here
     = note: the full type name has been written to '/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/typenum-7879abad6194cea8.long-type-4449308955079643001.txt'
     = note: 122 redundant requirements hidden
     = note: required for `UInt<UInt<uint::UTerm, bit::B1>, B0>` to implement `Shl<UInt<_, _>>`
note: required for `uint::UTerm` to implement `SetBit<UInt<_, _>, bit::B1>`
    --> /cargo/registry/src/index.crates.io-6f17d22bba15001f/typenum-1.16.0/src/uint.rs:1546:15
     |
1546 | impl<N, I, B> SetBit<I, B> for N
...
...
1549 |     PrivateSetBitOut<N, I, B>: Trim,
     |                                ---- unsatisfied trait bound introduced here
note: required for `()` to implement `PrivateDivIf<UInt<Ul, Bl>, UInt<Ur, Br>, uint::UTerm, _, UInt<_, _>, Equal>`
    --> /cargo/registry/src/index.crates.io-6f17d22bba15001f/typenum-1.16.0/src/uint.rs:1899:26
     |
1899 | impl<N, D, Q, R, Ui, Bi> PrivateDivIf<N, D, Q, R, UInt<Ui, Bi>, Equal> for ()
...
...
1903 |     (): PrivateDiv<N, D, SetBitOut<Q, UInt<Ui, Bi>, B1>, U0, Sub1<UInt<Ui, Bi>>>,
     |         ------------------------------------------------------------------------ unsatisfied trait bound introduced here
note: required for `()` to implement `PrivateDiv<UInt<Ul, Bl>, UInt<Ur, Br>, uint::UTerm, uint::UTerm, <_ as Sub<bit::B1>>::Output>`
    --> /cargo/registry/src/index.crates.io-6f17d22bba15001f/typenum-1.16.0/src/uint.rs:1748:18
     |
1748 |   impl<N, D, Q, I> PrivateDiv<N, D, Q, U0, I> for ()
...
...
1753 |       (): PrivateDivIf<
1754 | |         N,
1755 | |         D,
1756 | |         Q,
...    |
...    |
1759 | |         Compare<TrimOut<UInt<UTerm, GetBitOut<N, I>>>, D>,
1760 | |     >,
     | |_____- unsatisfied trait bound introduced here
For more information about this error, try `rustc --explain E0275`.
error: could not compile `typenum` due to previous error
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:04:34
