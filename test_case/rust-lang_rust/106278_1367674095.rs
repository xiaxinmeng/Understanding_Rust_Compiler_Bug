plain
   Compiling rand_core v0.6.2
   Compiling parking_lot v0.11.2
   Compiling psm v0.1.21
   Compiling stacker v0.1.15
error[E0275]: overflow evaluating the requirement `UInt<UInt<UInt<UInt<UInt<UInt<UInt<..., ...>, ...>, ...>, ...>, ...>, ...>, ...>: marker_traits::Unsigned`
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1717:12
     |
1717 |         ().private_div_quotient(self, rhs, U0::new(), U0::new(), self.len() - B1)
     |
     = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`typenum`)
     = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`typenum`)
note: required for `UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<..., ...>, ...>, ...>, ...>, ...>, ...>, ...>, ...>, ...>` to implement `type_operators::Cmp<uint::UTerm>`
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1100:27
     |
1100 | impl<U: Unsigned, B: Bit> Cmp<UTerm> for UInt<U, B> {
     |                           ^^^^^^^^^^     ^^^^^^^^^^
     = note: the full type name has been written to '/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/typenum-5b9f573fdb1ebb7f.long-type-15896083007198880753.txt'
note: required for `()` to implement `PrivateDiv<_, uint::UTerm, _, UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<_, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>`
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1810:26
     |
1810 | impl<N, D, Q, Ur, Br, I> PrivateDiv<N, D, Q, UInt<Ur, Br>, I> for ()
     |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^     ^^
note: required for `()` to implement `PrivateDivIf<_, uint::UTerm, _, UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<_, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, UInt<_, _>, Less>`
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1867:26
     |
1867 | impl<N, D, Q, R, Ui, Bi> PrivateDivIf<N, D, Q, R, UInt<Ui, Bi>, Less> for ()
     = note: 125 redundant requirements hidden
     = note: 125 redundant requirements hidden
     = note: required for `()` to implement `PrivateDiv<_, uint::UTerm, _, UInt<_, _>, UInt<_, _>>`

error[E0275]: overflow evaluating the requirement `UInt<UInt<UInt<UInt<UInt<UInt<UInt<..., ...>, ...>, ...>, ...>, ...>, ...>, ...>: marker_traits::Unsigned`
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1744:12
     |
1744 |         ().private_div_remainder(self, rhs, UTerm, UTerm, self.len() - B1)
     |
     = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`typenum`)
     = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`typenum`)
note: required for `UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<..., ...>, ...>, ...>, ...>, ...>, ...>, ...>, ...>, ...>` to implement `type_operators::Cmp<uint::UTerm>`
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1100:27
     |
1100 | impl<U: Unsigned, B: Bit> Cmp<UTerm> for UInt<U, B> {
     |                           ^^^^^^^^^^     ^^^^^^^^^^
     = note: the full type name has been written to '/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/typenum-5b9f573fdb1ebb7f.long-type-15896083007198880753.txt'
note: required for `()` to implement `PrivateDiv<_, uint::UTerm, _, UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<_, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>`
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1810:26
     |
1810 | impl<N, D, Q, Ur, Br, I> PrivateDiv<N, D, Q, UInt<Ur, Br>, I> for ()
     |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^     ^^
note: required for `()` to implement `PrivateDivIf<_, uint::UTerm, _, UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<_, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, UInt<_, _>, Less>`
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1867:26
     |
1867 | impl<N, D, Q, R, Ui, Bi> PrivateDivIf<N, D, Q, R, UInt<Ui, Bi>, Less> for ()
     = note: 125 redundant requirements hidden
     = note: 125 redundant requirements hidden
     = note: required for `()` to implement `PrivateDiv<_, uint::UTerm, _, UInt<_, _>, UInt<_, _>>`
   Compiling hashbrown v0.12.3
   Compiling rand_chacha v0.3.0
   Compiling rand_xoshiro v0.6.0
   Compiling measureme v10.1.0
   Compiling measureme v10.1.0
error[E0275]: overflow evaluating the requirement `UInt<UInt<UInt<UInt<UInt<UInt<UInt<..., ...>, ...>, ...>, ...>, ...>, ...>, ...>: type_operators::Cmp<...>`
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1794:12
     |
1794 |         ().private_div_if_quotient(n, d, q, r, i, r_cmp_d)
     |
     = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`typenum`)
     = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`typenum`)
note: required for `()` to implement `PrivateDiv<_, uint::UTerm, _, UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<_, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>`
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1810:26
     |
1810 | impl<N, D, Q, Ur, Br, I> PrivateDiv<N, D, Q, UInt<Ur, Br>, I> for ()
     |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^     ^^
note: required for `()` to implement `PrivateDivIf<_, uint::UTerm, _, UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<_, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, UInt<_, _>, Less>`
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1867:26
     |
1867 | impl<N, D, Q, R, Ui, Bi> PrivateDivIf<N, D, Q, R, UInt<Ui, Bi>, Less> for ()
     = note: 126 redundant requirements hidden
     = note: 126 redundant requirements hidden
     = note: required for `()` to implement `PrivateDivIf<_, uint::UTerm, _, UInt<_, _>, UInt<_, _>, Less>`
   Compiling odht v0.3.1
   Compiling termize v0.1.1
   Compiling termize v0.1.1
error[E0275]: overflow evaluating the requirement `UInt<UInt<UInt<UInt<UInt<UInt<UInt<..., ...>, ...>, ...>, ...>, ...>, ...>, ...>: type_operators::Cmp<...>`
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1805:12
     |
1805 |         ().private_div_if_remainder(n, d, q, r, i, r_cmp_d)
     |
     = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`typenum`)
     = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`typenum`)
note: required for `()` to implement `PrivateDiv<_, uint::UTerm, _, UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<_, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>`
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1810:26
     |
1810 | impl<N, D, Q, Ur, Br, I> PrivateDiv<N, D, Q, UInt<Ur, Br>, I> for ()
     |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^     ^^
note: required for `()` to implement `PrivateDivIf<_, uint::UTerm, _, UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<_, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, _>, UInt<_, _>, Less>`
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1867:26
     |
1867 | impl<N, D, Q, R, Ui, Bi> PrivateDivIf<N, D, Q, R, UInt<Ui, Bi>, Less> for ()
     = note: 126 redundant requirements hidden
     = note: 126 redundant requirements hidden
     = note: required for `()` to implement `PrivateDivIf<_, uint::UTerm, _, UInt<_, _>, UInt<_, _>, Less>`
For more information about this error, try `rustc --explain E0275`.
error: could not compile `typenum` due to 4 previous errors
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:05:26
