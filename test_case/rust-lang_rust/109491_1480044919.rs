plain
[RUSTC-TIMING] ppv_lite86 test:false 0.609
   Compiling parking_lot v0.11.2
[RUSTC-TIMING] unic_emoji_char test:false 0.525
   Compiling hashbrown v0.12.3
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
     = note: the full type name has been written to '/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/typenum-2906afe466d9154f.long-type-1771982007099492944.txt'
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
[RUSTC-TIMING] proc_macro_hack test:false 1.567
[RUSTC-TIMING] typenum test:false 0.620
error: could not compile `typenum` due to previous error
---
Total duration:                           9m 48s
------------------------------------------------
root INFO: Free disk space: 508.20 GiB out of total 581.32 GiB (12.58% used)
Traceback (most recent call last):
  File "../src/ci/stage-build.py", line 839, in <module>
    raise e
  File "../src/ci/stage-build.py", line 836, in <module>
    execute_build_pipeline(timer, pipeline, build_args)
  File "../src/ci/stage-build.py", line 760, in execute_build_pipeline
    LLVM_PROFILE_DIR=str(pipeline.llvm_profile_dir_root() / "prof-%p")
  File "../src/ci/stage-build.py", line 571, in build_rustc
    cmd(arguments, env=env)
  File "../src/ci/stage-build.py", line 452, in cmd
    return subprocess.run(args, env=environment, check=True)
  File "/usr/lib64/python3.6/subprocess.py", line 438, in run
    output=stdout, stderr=stderr)
subprocess.CalledProcessError: Command '['/usr/bin/python3', '/checkout/x.py', 'build', '--target', 'x86_64-unknown-linux-gnu', '--host', 'x86_64-unknown-linux-gnu', '--stage', '2', 'library/std', '--llvm-profile-generate']' returned non-zero exit status 1.
