plain
   Compiling crossbeam-utils v0.8.3
   Compiling num-traits v0.2.12
   Compiling num-integer v0.1.43
   Compiling unicode-normalization v0.1.13
error: environment variable `TYPENUM_BUILD_OP` not defined
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/lib.rs:76:14
    |
76  |       include!(env!("TYPENUM_BUILD_OP"));
    |                ^^^^^^^^^^^^^^^^^^^^^^^^ in this macro invocation
   ::: /checkout/library/core/src/macros/mod.rs:881:5
    |
881 | /     macro_rules! env {
881 | /     macro_rules! env {
882 | |         ($name:expr $(,)?) => {{ /* compiler built-in */ }};
883 | |         ($name:expr, $error_msg:expr $(,)?) => {{ /* compiler built-in */ }};
    | |_____- in this expansion of `env!`


error: environment variable `TYPENUM_BUILD_CONSTS` not defined
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/lib.rs:77:14
    |
77  |       include!(env!("TYPENUM_BUILD_CONSTS"));
    |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ in this macro invocation
   ::: /checkout/library/core/src/macros/mod.rs:881:5
    |
881 | /     macro_rules! env {
881 | /     macro_rules! env {
882 | |         ($name:expr $(,)?) => {{ /* compiler built-in */ }};
883 | |         ($name:expr, $error_msg:expr $(,)?) => {{ /* compiler built-in */ }};
    | |_____- in this expansion of `env!`

error[E0432]: unresolved import `consts`
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/lib.rs:90:9
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/lib.rs:90:9
   |
90 | pub use consts::*;
   |         ^^^^^^
   |         |
   |         unresolved import
   |         help: a similar path exists: `core::f32::consts`

error[E0432]: unresolved imports `generated::consts`, `Cmp`, `Equal`, `Greater`, `Less`, `NonZero`, `PowerOfTwo`, `Min`, `Max`, `consts::N1`, `consts::P1`, `consts::U0`, `consts::U1`, `Cmp`, `Equal`, `Greater`, `Less`, `NonZero`, `Pow`, `PowerOfTwo`, `PartialDiv`, `Quot`, `Gcd`, `Gcf`, `Max`, `Maximum`, `Min`, `Minimum`, `Equal`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/lib.rs:91:9
91  | pub use generated::consts;
91  | pub use generated::consts;
    |         ^^^^^^^^^^^^^^^^^ no `consts` in `generated`
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:15:6
    |
    |
15  | use {Cmp, Equal, Greater, Less, NonZero, PowerOfTwo};
...
251 | use Min;
    |     ^^^
...
...
281 | use Max;
    |     ^^^
    | 
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/int.rs:33:14
    |
33  | use consts::{N1, P1, U0, U1};
    |              ^^  ^^  ^^  ^^
...
37  | use {Cmp, Equal, Greater, Less, NonZero, Pow, PowerOfTwo};
...
...
613 | use {PartialDiv, Quot};
...
...
891 | use {Gcd, Gcf};
...
...
963 | use {Max, Maximum, Min, Minimum};
    | 
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/private.rs:392:6
    |
    |
392 | use {Equal, False, Greater, Less, True};


error[E0432]: unresolved imports `False`, `Greater`, `Less`, `True`, `Bit`, `NInt`, `NonZero`, `PInt`, `UInt`, `UTerm`, `Unsigned`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/private.rs:392:13
    |
392 | use {Equal, False, Greater, Less, True};
    |             ^^^^^  ^^^^^^^  ^^^^  ^^^^ no `True` in the root
    |             |
    |             no `False` in the root
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/type_operators.rs:5:6
    |
    |
5   | use {Bit, NInt, NonZero, PInt, UInt, UTerm, Unsigned, Z0};


error[E0432]: unresolved imports `Z0`, `Compare`, `Cmp`, `Equal`, `Gcd`, `Greater`, `IsGreaterOrEqual`, `Len`, `Less`, `Logarithm2`, `Maximum`, `Minimum`, `NonZero`, `Ord`, `Pow`, `SquareRoot`, `consts::U0`, `consts::U1`, `Add1`, `Double`, `Gcf`, `GrEq`, `Length`, `Log2`, `Or`, `Prod`, `Shleft`, `Shright`, `Sqrt`, `Square`, `Sub1`, `Sum`, `Compare`, `Diff`, `PartialDiv`, `Quot`, `Min`, `Max`
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/type_operators.rs:5:55
     |
5    | use {Bit, NInt, NonZero, PInt, UInt, UTerm, Unsigned, Z0};
     |                                                       ^^ no `Z0` in the root
323  | use Compare;
     |     ^^^^^^^
     | 
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:33:5
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:33:5
     |
33   |     Cmp, Equal, Gcd, Greater, IsGreaterOrEqual, Len, Less, Logarithm2, Maximum, Minimum, NonZero,
     |     ^^^  ^^^^^  ^^^  ^^^^^^^  ^^^^^^^^^^^^^^^^  ^^^  ^^^^  ^^^^^^^^^^  ^^^^^^^  ^^^^^^^  ^^^^^^^
34   |     Ord, Pow, SquareRoot,
...
...
50   | use consts::{U0, U1};
     |              ^^  ^^
51   | use {Add1, Double, Gcf, GrEq, Length, Log2, Or, Prod, Shleft, Shright, Sqrt, Square, Sub1, Sum};
     |      ^^^^  ^^^^^^  ^^^  ^^^^  ^^^^^^  ^^^^  ^^  ^^^^  ^^^^^^  ^^^^^^^  ^^^^  ^^^^^^  ^^^^  ^^^
1752 | use Compare;
     |     ^^^^^^^
...
1943 | use Diff;
1943 | use Diff;
     |     ^^^^
...
2052 | use {PartialDiv, Quot};
...
2118 | use Min;
     |     ^^^
...
...
2190 | use Max;
     |     ^^^

error: cannot determine resolution for the derive macro `Ord`
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:57:25
   |
57 | #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug, Default)]
   |
   |
   = note: import resolution is stuck, try simplifying macro imports

error: cannot determine resolution for the derive macro `Ord`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:154:25
    |
154 | #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug, Default)]
    |
    |
    = note: import resolution is stuck, try simplifying macro imports
error[E0433]: failed to resolve: use of undeclared type `U1`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/int.rs:586:25
    |
550 | / macro_rules! impl_int_div {
550 | / macro_rules! impl_int_div {
551 | |     ($A:ident, $B:ident, $R:ident) => {
552 | |         /// `$A<Ul> / $B<Ur> = $R<Ul / Ur>`
553 | |         impl<Ul: Unsigned + NonZero, Ur: Unsigned + NonZero> Div<$B<Ur>> for $A<Ul>
...   |
586 | |                 $R { n: U1::new() }
    | |                         ^^ use of undeclared type `U1`
602 | |     };
603 | | }
    | |_- in this expansion of `impl_int_div!`
604 | 
604 | 
605 |   impl_int_div!(PInt, PInt, PInt);

error[E0433]: failed to resolve: use of undeclared type `U1`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/int.rs:586:25
    |
    |
550 | / macro_rules! impl_int_div {
551 | |     ($A:ident, $B:ident, $R:ident) => {
552 | |         /// `$A<Ul> / $B<Ur> = $R<Ul / Ur>`
553 | |         impl<Ul: Unsigned + NonZero, Ur: Unsigned + NonZero> Div<$B<Ur>> for $A<Ul>
...   |
586 | |                 $R { n: U1::new() }
    | |                         ^^ use of undeclared type `U1`
602 | |     };
603 | | }
    | |_- in this expansion of `impl_int_div!`
...
...
606 |   impl_int_div!(PInt, NInt, NInt);

error[E0433]: failed to resolve: use of undeclared type `U1`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/int.rs:586:25
    |
    |
550 | / macro_rules! impl_int_div {
551 | |     ($A:ident, $B:ident, $R:ident) => {
552 | |         /// `$A<Ul> / $B<Ur> = $R<Ul / Ur>`
553 | |         impl<Ul: Unsigned + NonZero, Ur: Unsigned + NonZero> Div<$B<Ur>> for $A<Ul>
...   |
586 | |                 $R { n: U1::new() }
    | |                         ^^ use of undeclared type `U1`
602 | |     };
603 | | }
    | |_- in this expansion of `impl_int_div!`
...
...
607 |   impl_int_div!(NInt, PInt, NInt);

error[E0433]: failed to resolve: use of undeclared type `U1`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/int.rs:586:25
    |
    |
550 | / macro_rules! impl_int_div {
551 | |     ($A:ident, $B:ident, $R:ident) => {
552 | |         /// `$A<Ul> / $B<Ur> = $R<Ul / Ur>`
553 | |         impl<Ul: Unsigned + NonZero, Ur: Unsigned + NonZero> Div<$B<Ur>> for $A<Ul>
...   |
586 | |                 $R { n: U1::new() }
    | |                         ^^ use of undeclared type `U1`
602 | |     };
603 | | }
    | |_- in this expansion of `impl_int_div!`
...
...
608 |   impl_int_div!(NInt, NInt, PInt);

error[E0433]: failed to resolve: use of undeclared type `P1`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/int.rs:783:9
    |
---

error[E0433]: failed to resolve: use of undeclared type `U1`
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1312:26
     |
1312 |         self.private_pow(U1::new(), n)
     |                          ^^ use of undeclared type `U1`
error[E0433]: failed to resolve: use of undeclared type `U1`
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1614:29
     |
     |
1614 |         <U1 as Shl<I>>::shl(U1::new(), i)
     |                             ^^ use of undeclared type `U1`
error[E0433]: failed to resolve: use of undeclared type `U0`
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1717:44
     |
     |
1717 |         ().private_div_quotient(self, rhs, U0::new(), U0::new(), self.len() - B1)
     |                                            ^^ use of undeclared type `U0`
error[E0433]: failed to resolve: use of undeclared type `U0`
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1717:55
     |
     |
1717 |         ().private_div_quotient(self, rhs, U0::new(), U0::new(), self.len() - B1)
     |                                                       ^^ use of undeclared type `U0`
error[E0433]: failed to resolve: use of undeclared type `U0`
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1925:69
     |
     |
1925 |         ().private_div_quotient(n, d, q.set_bit::<Internal>(i, B1), U0::new(), i - B1)
     |                                                                     ^^ use of undeclared type `U0`
error[E0433]: failed to resolve: use of undeclared type `U0`
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1939:70
     |
     |
1939 |         ().private_div_remainder(n, d, q.set_bit::<Internal>(i, B1), U0::new(), i - B1)
     |                                                                      ^^ use of undeclared type `U0`
error[E0412]: cannot find type `U1` in this scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/int.rs:582:30
    |
550 | / macro_rules! impl_int_div {
550 | / macro_rules! impl_int_div {
551 | |     ($A:ident, $B:ident, $R:ident) => {
552 | |         /// `$A<Ul> / $B<Ur> = $R<Ul / Ur>`
553 | |         impl<Ul: Unsigned + NonZero, Ur: Unsigned + NonZero> Div<$B<Ur>> for $A<Ul>
...   |
582 | |             type Output = $R<U1>;
    | |                              ^^ help: a struct with a similar name exists: `B1`
602 | |     };
603 | | }
    | |_- in this expansion of `impl_int_div!`
604 | 
604 | 
605 |   impl_int_div!(PInt, PInt, PInt);
    | 
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:33:1
    |
33  |   pub struct B1;
33  |   pub struct B1;
    |   -------------- similarly named struct `B1` defined here

error[E0412]: cannot find type `U1` in this scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/int.rs:582:30
    |
550 | / macro_rules! impl_int_div {
551 | |     ($A:ident, $B:ident, $R:ident) => {
552 | |         /// `$A<Ul> / $B<Ur> = $R<Ul / Ur>`
553 | |         impl<Ul: Unsigned + NonZero, Ur: Unsigned + NonZero> Div<$B<Ur>> for $A<Ul>
...   |
582 | |             type Output = $R<U1>;
    | |                              ^^ help: a struct with a similar name exists: `B1`
602 | |     };
603 | | }
    | |_- in this expansion of `impl_int_div!`
...
...
606 |   impl_int_div!(PInt, NInt, NInt);
    | 
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:33:1
    |
33  |   pub struct B1;
33  |   pub struct B1;
    |   -------------- similarly named struct `B1` defined here

error[E0412]: cannot find type `U1` in this scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/int.rs:582:30
    |
550 | / macro_rules! impl_int_div {
551 | |     ($A:ident, $B:ident, $R:ident) => {
552 | |         /// `$A<Ul> / $B<Ur> = $R<Ul / Ur>`
553 | |         impl<Ul: Unsigned + NonZero, Ur: Unsigned + NonZero> Div<$B<Ur>> for $A<Ul>
...   |
582 | |             type Output = $R<U1>;
    | |                              ^^ help: a struct with a similar name exists: `B1`
602 | |     };
603 | | }
    | |_- in this expansion of `impl_int_div!`
...
...
607 |   impl_int_div!(NInt, PInt, NInt);
    | 
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:33:1
    |
33  |   pub struct B1;
33  |   pub struct B1;
    |   -------------- similarly named struct `B1` defined here

error[E0412]: cannot find type `U1` in this scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/int.rs:582:30
    |
550 | / macro_rules! impl_int_div {
551 | |     ($A:ident, $B:ident, $R:ident) => {
552 | |         /// `$A<Ul> / $B<Ur> = $R<Ul / Ur>`
553 | |         impl<Ul: Unsigned + NonZero, Ur: Unsigned + NonZero> Div<$B<Ur>> for $A<Ul>
...   |
582 | |             type Output = $R<U1>;
    | |                              ^^ help: a struct with a similar name exists: `B1`
602 | |     };
603 | | }
    | |_- in this expansion of `impl_int_div!`
...
...
608 |   impl_int_div!(NInt, NInt, PInt);
    | 
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:33:1
    |
33  |   pub struct B1;
33  |   pub struct B1;
    |   -------------- similarly named struct `B1` defined here

error[E0412]: cannot find type `U0` in this scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/int.rs:745:73
    |
731 | / macro_rules! impl_int_rem {
732 | |     ($A:ident, $B:ident, $R:ident) => {
733 | |         /// `$A<Ul> % $B<Ur> = $R<Ul % Ur>`
734 | |         impl<Ul: Unsigned + NonZero, Ur: Unsigned + NonZero> Rem<$B<Ur>> for $A<Ul>
...   |
745 | |         impl<Ul: Unsigned + NonZero, Ur: Unsigned + NonZero> PrivateRem<U0, $B<Ur>> for $A<Ul> {
    | |                                                                         ^^ help: a struct with a similar name exists: `B0`
767 | |     };
768 | | }
    | |_- in this expansion of `impl_int_rem!`
769 | 
769 | 
770 |   impl_int_rem!(PInt, PInt, PInt);
    | 
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:21:1
    |
21  |   pub struct B0;
21  |   pub struct B0;
    |   -------------- similarly named struct `B0` defined here

error[E0412]: cannot find type `U0` in this scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/int.rs:749:37
    |
731 | / macro_rules! impl_int_rem {
732 | |     ($A:ident, $B:ident, $R:ident) => {
733 | |         /// `$A<Ul> % $B<Ur> = $R<Ul % Ur>`
734 | |         impl<Ul: Unsigned + NonZero, Ur: Unsigned + NonZero> Rem<$B<Ur>> for $A<Ul>
...   |
749 | |             fn private_rem(self, _: U0, _: $B<Ur>) -> Self::Output {
    | |                                     ^^ help: a struct with a similar name exists: `B0`
767 | |     };
768 | | }
    | |_- in this expansion of `impl_int_rem!`
769 | 
769 | 
770 |   impl_int_rem!(PInt, PInt, PInt);
    | 
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:21:1
    |
21  |   pub struct B0;
21  |   pub struct B0;
    |   -------------- similarly named struct `B0` defined here

error[E0412]: cannot find type `U0` in this scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/int.rs:745:73
    |
731 | / macro_rules! impl_int_rem {
732 | |     ($A:ident, $B:ident, $R:ident) => {
733 | |         /// `$A<Ul> % $B<Ur> = $R<Ul % Ur>`
734 | |         impl<Ul: Unsigned + NonZero, Ur: Unsigned + NonZero> Rem<$B<Ur>> for $A<Ul>
...   |
745 | |         impl<Ul: Unsigned + NonZero, Ur: Unsigned + NonZero> PrivateRem<U0, $B<Ur>> for $A<Ul> {
    | |                                                                         ^^ help: a struct with a similar name exists: `B0`
767 | |     };
768 | | }
    | |_- in this expansion of `impl_int_rem!`
...
...
771 |   impl_int_rem!(PInt, NInt, PInt);
    | 
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:21:1
    |
21  |   pub struct B0;
21  |   pub struct B0;
    |   -------------- similarly named struct `B0` defined here

error[E0412]: cannot find type `U0` in this scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/int.rs:749:37
    |
731 | / macro_rules! impl_int_rem {
732 | |     ($A:ident, $B:ident, $R:ident) => {
733 | |         /// `$A<Ul> % $B<Ur> = $R<Ul % Ur>`
734 | |         impl<Ul: Unsigned + NonZero, Ur: Unsigned + NonZero> Rem<$B<Ur>> for $A<Ul>
...   |
749 | |             fn private_rem(self, _: U0, _: $B<Ur>) -> Self::Output {
    | |                                     ^^ help: a struct with a similar name exists: `B0`
767 | |     };
768 | | }
    | |_- in this expansion of `impl_int_rem!`
...
...
771 |   impl_int_rem!(PInt, NInt, PInt);
    | 
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:21:1
    |
21  |   pub struct B0;
21  |   pub struct B0;
    |   -------------- similarly named struct `B0` defined here

error[E0412]: cannot find type `U0` in this scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/int.rs:745:73
    |
731 | / macro_rules! impl_int_rem {
732 | |     ($A:ident, $B:ident, $R:ident) => {
733 | |         /// `$A<Ul> % $B<Ur> = $R<Ul % Ur>`
734 | |         impl<Ul: Unsigned + NonZero, Ur: Unsigned + NonZero> Rem<$B<Ur>> for $A<Ul>
...   |
745 | |         impl<Ul: Unsigned + NonZero, Ur: Unsigned + NonZero> PrivateRem<U0, $B<Ur>> for $A<Ul> {
    | |                                                                         ^^ help: a struct with a similar name exists: `B0`
767 | |     };
768 | | }
    | |_- in this expansion of `impl_int_rem!`
...
...
772 |   impl_int_rem!(NInt, PInt, NInt);
    | 
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:21:1
    |
21  |   pub struct B0;
21  |   pub struct B0;
    |   -------------- similarly named struct `B0` defined here

error[E0412]: cannot find type `U0` in this scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/int.rs:749:37
    |
731 | / macro_rules! impl_int_rem {
732 | |     ($A:ident, $B:ident, $R:ident) => {
733 | |         /// `$A<Ul> % $B<Ur> = $R<Ul % Ur>`
734 | |         impl<Ul: Unsigned + NonZero, Ur: Unsigned + NonZero> Rem<$B<Ur>> for $A<Ul>
...   |
749 | |             fn private_rem(self, _: U0, _: $B<Ur>) -> Self::Output {
    | |                                     ^^ help: a struct with a similar name exists: `B0`
767 | |     };
768 | | }
    | |_- in this expansion of `impl_int_rem!`
...
...
772 |   impl_int_rem!(NInt, PInt, NInt);
    | 
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:21:1
    |
21  |   pub struct B0;
21  |   pub struct B0;
    |   -------------- similarly named struct `B0` defined here

error[E0412]: cannot find type `U0` in this scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/int.rs:745:73
    |
731 | / macro_rules! impl_int_rem {
732 | |     ($A:ident, $B:ident, $R:ident) => {
733 | |         /// `$A<Ul> % $B<Ur> = $R<Ul % Ur>`
734 | |         impl<Ul: Unsigned + NonZero, Ur: Unsigned + NonZero> Rem<$B<Ur>> for $A<Ul>
...   |
745 | |         impl<Ul: Unsigned + NonZero, Ur: Unsigned + NonZero> PrivateRem<U0, $B<Ur>> for $A<Ul> {
    | |                                                                         ^^ help: a struct with a similar name exists: `B0`
767 | |     };
768 | | }
    | |_- in this expansion of `impl_int_rem!`
...
...
773 |   impl_int_rem!(NInt, NInt, NInt);
    | 
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:21:1
    |
21  |   pub struct B0;
21  |   pub struct B0;
    |   -------------- similarly named struct `B0` defined here

error[E0412]: cannot find type `U0` in this scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/int.rs:749:37
    |
731 | / macro_rules! impl_int_rem {
732 | |     ($A:ident, $B:ident, $R:ident) => {
733 | |         /// `$A<Ul> % $B<Ur> = $R<Ul % Ur>`
734 | |         impl<Ul: Unsigned + NonZero, Ur: Unsigned + NonZero> Rem<$B<Ur>> for $A<Ul>
...   |
749 | |             fn private_rem(self, _: U0, _: $B<Ur>) -> Self::Output {
    | |                                     ^^ help: a struct with a similar name exists: `B0`
767 | |     };
768 | | }
    | |_- in this expansion of `impl_int_rem!`
...
...
773 |   impl_int_rem!(NInt, NInt, NInt);
    | 
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:21:1
    |
21  |   pub struct B0;
---

error[E0412]: cannot find type `P1` in this scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/int.rs:806:46
    |
806 | impl<U: Unsigned + NonZero> Pow<NInt<U>> for P1 {
    |                                              ^^ help: a struct with a similar name exists: `B1`
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:33:1
    |
33  | pub struct B1;
    | -------------- similarly named struct `B1` defined here
---

error[E0412]: cannot find type `N1` in this scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/int.rs:815:46
    |
815 | impl<U: Unsigned> Pow<NInt<UInt<U, B0>>> for N1 {
    |                                              ^^ help: a struct with a similar name exists: `B1`
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:33:1
    |
33  | pub struct B1;
    | -------------- similarly named struct `B1` defined here
---

error[E0412]: cannot find type `N1` in this scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/int.rs:824:46
    |
824 | impl<U: Unsigned> Pow<NInt<UInt<U, B1>>> for N1 {
    |                                              ^^ help: a struct with a similar name exists: `B1`
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:33:1
    |
33  | pub struct B1;
    | -------------- similarly named struct `B1` defined here
---

error[E0412]: cannot find type `U1` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1307:19
     |
1307 |     X: PrivatePow<U1, N>,
     | 
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:33:1
     |
33   | pub struct B1;
33   | pub struct B1;
     | -------------- similarly named struct `B1` defined here
     |
help: a struct with a similar name exists
     |
1307 |     X: PrivatePow<B1, N>,
help: you might be missing a type parameter
     |
     |
1305 | impl<X: Unsigned, N: Unsigned, U1> Pow<N> for X

error[E0412]: cannot find type `U1` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1309:36
     |
     |
1309 |     type Output = PrivatePowOut<X, U1, N>;
     | 
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:33:1
     |
33   | pub struct B1;
33   | pub struct B1;
     | -------------- similarly named struct `B1` defined here
     |
help: a struct with a similar name exists
     |
1309 |     type Output = PrivatePowOut<X, B1, N>;
help: you might be missing a type parameter
     |
     |
1305 | impl<X: Unsigned, N: Unsigned, U1> Pow<N> for X

error[E0412]: cannot find type `U0` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1316:46
     |
     |
1316 | impl<Y: Unsigned, X: Unsigned> PrivatePow<Y, U0> for X {
     | 
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:21:1
     |
21   | pub struct B0;
21   | pub struct B0;
     | -------------- similarly named struct `B0` defined here
     |
help: a struct with a similar name exists
     |
1316 | impl<Y: Unsigned, X: Unsigned> PrivatePow<Y, B0> for X {
help: you might be missing a type parameter
     |
     |
1316 | impl<Y: Unsigned, X: Unsigned, U0> PrivatePow<Y, U0> for X {

error[E0412]: cannot find type `U0` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1320:35
     |
     |
1320 |     fn private_pow(self, y: Y, _: U0) -> Self::Output {
     |                                   ^^ help: a struct with a similar name exists: `B0`
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:21:1
     |
21   | pub struct B0;
     | -------------- similarly named struct `B0` defined here
     | -------------- similarly named struct `B0` defined here

error[E0412]: cannot find type `U1` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1325:46
     |
1325 | impl<Y: Unsigned, X: Unsigned> PrivatePow<Y, U1> for X
     | 
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:33:1
     |
33   | pub struct B1;
33   | pub struct B1;
     | -------------- similarly named struct `B1` defined here
     |
help: a struct with a similar name exists
     |
1325 | impl<Y: Unsigned, X: Unsigned> PrivatePow<Y, B1> for X
help: you might be missing a type parameter
     |
     |
1325 | impl<Y: Unsigned, X: Unsigned, U1> PrivatePow<Y, U1> for X

error[E0412]: cannot find type `U1` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1332:35
     |
     |
1332 |     fn private_pow(self, y: Y, _: U1) -> Self::Output {
     |                                   ^^ help: a struct with a similar name exists: `B1`
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:33:1
     |
33   | pub struct B1;
     | -------------- similarly named struct `B1` defined here
     | -------------- similarly named struct `B1` defined here

error[E0412]: cannot find type `U0` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1376:10
     |
1376 | impl Gcd<U0> for U0 {
     | 
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:21:1
     |
21   | pub struct B0;
21   | pub struct B0;
     | -------------- similarly named struct `B0` defined here
     |
help: a struct with a similar name exists
     |
1376 | impl Gcd<B0> for U0 {
help: you might be missing a type parameter
     |
     |
1376 | impl<U0> Gcd<U0> for U0 {

error[E0412]: cannot find type `U0` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1376:18
     |
     |
1376 | impl Gcd<U0> for U0 {
     |                  ^^ help: a struct with a similar name exists: `B0`
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:21:1
     |
21   | pub struct B0;
     | -------------- similarly named struct `B0` defined here
---

error[E0412]: cannot find type `U0` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1381:13
     |
1381 | impl<X> Gcd<U0> for X
     | 
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:21:1
     |
21   | pub struct B0;
21   | pub struct B0;
     | -------------- similarly named struct `B0` defined here
     |
help: a struct with a similar name exists
     |
1381 | impl<X> Gcd<B0> for X
help: you might be missing a type parameter
     |
     |
1381 | impl<X, U0> Gcd<U0> for X

error[E0412]: cannot find type `U0` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1389:20
     |
     |
1389 | impl<Y> Gcd<Y> for U0
     |                    ^^ help: a struct with a similar name exists: `B0`
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:21:1
     |
21   | pub struct B0;
     | -------------- similarly named struct `B0` defined here
     | -------------- similarly named struct `B0` defined here

error[E0412]: cannot find type `U0` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1485:21
     |
1485 | impl<Un, Bn> GetBit<U0> for UInt<Un, Bn>
     | 
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:21:1
     |
21   | pub struct B0;
21   | pub struct B0;
     | -------------- similarly named struct `B0` defined here
     |
help: a struct with a similar name exists
     |
1485 | impl<Un, Bn> GetBit<B0> for UInt<Un, Bn>
help: you might be missing a type parameter
     |
     |
1485 | impl<Un, Bn, U0> GetBit<U0> for UInt<Un, Bn>

error[E0412]: cannot find type `U0` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1492:47
     |
     |
1492 |     fn get_bit<IM: InternalMarker>(&self, _: &U0) -> Self::Output {
     |                                               ^^ help: a struct with a similar name exists: `B0`
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:21:1
     |
21   | pub struct B0;
     | -------------- similarly named struct `B0` defined here
     | -------------- similarly named struct `B0` defined here

error[E0412]: cannot find type `U0` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1566:31
     |
1566 | impl<Un, Bn, B> PrivateSetBit<U0, B> for UInt<Un, Bn> {
     | 
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:21:1
     |
21   | pub struct B0;
21   | pub struct B0;
     | -------------- similarly named struct `B0` defined here
     |
help: a struct with a similar name exists
     |
1566 | impl<Un, Bn, B> PrivateSetBit<B0, B> for UInt<Un, Bn> {
help: you might be missing a type parameter
     |
     |
1566 | impl<Un, Bn, B, U0> PrivateSetBit<U0, B> for UInt<Un, Bn> {

error[E0412]: cannot find type `U0` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1570:33
     |
     |
1570 |     fn private_set_bit(self, _: U0, b: B) -> Self::Output {
     |                                 ^^ help: a struct with a similar name exists: `B0`
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:21:1
     |
21   | pub struct B0;
     | -------------- similarly named struct `B0` defined here
     | -------------- similarly named struct `B0` defined here

error[E0412]: cannot find type `U1` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1608:5
     |
1608 |     U1: Shl<I>,
     |     ^^ help: a struct with a similar name exists: `B1`
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:33:1
     |
33   | pub struct B1;
     | -------------- similarly named struct `B1` defined here
     | -------------- similarly named struct `B1` defined here

error[E0412]: cannot find type `U1` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1610:26
     |
1610 |     type Output = Shleft<U1, I>;
     | 
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:33:1
     |
33   | pub struct B1;
33   | pub struct B1;
     | -------------- similarly named struct `B1` defined here
     |
help: a struct with a similar name exists
     |
1610 |     type Output = Shleft<B1, I>;
help: you might be missing a type parameter
     |
     |
1606 | impl<I, U1> PrivateSetBit<I, B1> for UTerm

error[E0412]: cannot find type `U1` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1614:10
     |
     |
1614 |         <U1 as Shl<I>>::shl(U1::new(), i)
     |          ^^ help: a struct with a similar name exists: `B1`
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:33:1
     |
33   | pub struct B1;
     | -------------- similarly named struct `B1` defined here
     | -------------- similarly named struct `B1` defined here

error[E0412]: cannot find type `U0` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1711:48
     |
1711 |     (): PrivateDiv<UInt<Ul, Bl>, UInt<Ur, Br>, U0, U0, Sub1<Length<UInt<Ul, Bl>>>>,
     | 
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:21:1
     |
21   | pub struct B0;
21   | pub struct B0;
     | -------------- similarly named struct `B0` defined here
     |
help: a struct with a similar name exists
     |
1711 |     (): PrivateDiv<UInt<Ul, Bl>, UInt<Ur, Br>, B0, U0, Sub1<Length<UInt<Ul, Bl>>>>,
help: you might be missing a type parameter
     |
     |
1707 | impl<Ul: Unsigned, Bl: Bit, Ur: Unsigned, Br: Bit, U0> Div<UInt<Ur, Br>> for UInt<Ul, Bl>

error[E0412]: cannot find type `U0` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1711:52
     |
     |
1711 |     (): PrivateDiv<UInt<Ul, Bl>, UInt<Ur, Br>, U0, U0, Sub1<Length<UInt<Ul, Bl>>>>,
     | 
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:21:1
     |
21   | pub struct B0;
21   | pub struct B0;
     | -------------- similarly named struct `B0` defined here
     |
help: a struct with a similar name exists
     |
1711 |     (): PrivateDiv<UInt<Ul, Bl>, UInt<Ur, Br>, U0, B0, Sub1<Length<UInt<Ul, Bl>>>>,
help: you might be missing a type parameter
     |
     |
1707 | impl<Ul: Unsigned, Bl: Bit, Ur: Unsigned, Br: Bit, U0> Div<UInt<Ur, Br>> for UInt<Ul, Bl>

error[E0412]: cannot find type `U0` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1713:62
     |
     |
1713 |     type Output = PrivateDivQuot<UInt<Ul, Bl>, UInt<Ur, Br>, U0, U0, Sub1<Length<UInt<Ul, Bl>>>>;
     | 
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:21:1
     |
21   | pub struct B0;
21   | pub struct B0;
     | -------------- similarly named struct `B0` defined here
     |
help: a struct with a similar name exists
     |
1713 |     type Output = PrivateDivQuot<UInt<Ul, Bl>, UInt<Ur, Br>, B0, U0, Sub1<Length<UInt<Ul, Bl>>>>;
help: you might be missing a type parameter
     |
     |
1707 | impl<Ul: Unsigned, Bl: Bit, Ur: Unsigned, Br: Bit, U0> Div<UInt<Ur, Br>> for UInt<Ul, Bl>

error[E0412]: cannot find type `U0` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1713:66
     |
     |
1713 |     type Output = PrivateDivQuot<UInt<Ul, Bl>, UInt<Ur, Br>, U0, U0, Sub1<Length<UInt<Ul, Bl>>>>;
     | 
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:21:1
     |
21   | pub struct B0;
21   | pub struct B0;
     | -------------- similarly named struct `B0` defined here
     |
help: a struct with a similar name exists
     |
1713 |     type Output = PrivateDivQuot<UInt<Ul, Bl>, UInt<Ur, Br>, U0, B0, Sub1<Length<UInt<Ul, Bl>>>>;
help: you might be missing a type parameter
     |
     |
1707 | impl<Ul: Unsigned, Bl: Bit, Ur: Unsigned, Br: Bit, U0> Div<UInt<Ur, Br>> for UInt<Ul, Bl>

error[E0412]: cannot find type `U0` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1739:48
     |
     |
1739 |     (): PrivateDiv<UInt<Ul, Bl>, UInt<Ur, Br>, U0, U0, Sub1<Length<UInt<Ul, Bl>>>>,
     | 
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:21:1
     |
21   | pub struct B0;
21   | pub struct B0;
     | -------------- similarly named struct `B0` defined here
     |
help: a struct with a similar name exists
     |
1739 |     (): PrivateDiv<UInt<Ul, Bl>, UInt<Ur, Br>, B0, U0, Sub1<Length<UInt<Ul, Bl>>>>,
help: you might be missing a type parameter
     |
     |
1735 | impl<Ul: Unsigned, Bl: Bit, Ur: Unsigned, Br: Bit, U0> Rem<UInt<Ur, Br>> for UInt<Ul, Bl>

error[E0412]: cannot find type `U0` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1739:52
     |
     |
1739 |     (): PrivateDiv<UInt<Ul, Bl>, UInt<Ur, Br>, U0, U0, Sub1<Length<UInt<Ul, Bl>>>>,
     | 
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:21:1
     |
21   | pub struct B0;
21   | pub struct B0;
     | -------------- similarly named struct `B0` defined here
     |
help: a struct with a similar name exists
     |
1739 |     (): PrivateDiv<UInt<Ul, Bl>, UInt<Ur, Br>, U0, B0, Sub1<Length<UInt<Ul, Bl>>>>,
help: you might be missing a type parameter
     |
     |
1735 | impl<Ul: Unsigned, Bl: Bit, Ur: Unsigned, Br: Bit, U0> Rem<UInt<Ur, Br>> for UInt<Ul, Bl>

error[E0412]: cannot find type `U0` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1741:61
     |
     |
1741 |     type Output = PrivateDivRem<UInt<Ul, Bl>, UInt<Ur, Br>, U0, U0, Sub1<Length<UInt<Ul, Bl>>>>;
     | 
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:21:1
     |
21   | pub struct B0;
21   | pub struct B0;
     | -------------- similarly named struct `B0` defined here
     |
help: a struct with a similar name exists
     |
1741 |     type Output = PrivateDivRem<UInt<Ul, Bl>, UInt<Ur, Br>, B0, U0, Sub1<Length<UInt<Ul, Bl>>>>;
help: you might be missing a type parameter
     |
     |
1735 | impl<Ul: Unsigned, Bl: Bit, Ur: Unsigned, Br: Bit, U0> Rem<UInt<Ur, Br>> for UInt<Ul, Bl>

error[E0412]: cannot find type `U0` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1741:65
     |
     |
1741 |     type Output = PrivateDivRem<UInt<Ul, Bl>, UInt<Ur, Br>, U0, U0, Sub1<Length<UInt<Ul, Bl>>>>;
     | 
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:21:1
     |
21   | pub struct B0;
21   | pub struct B0;
     | -------------- similarly named struct `B0` defined here
     |
help: a struct with a similar name exists
     |
1741 |     type Output = PrivateDivRem<UInt<Ul, Bl>, UInt<Ur, Br>, U0, B0, Sub1<Length<UInt<Ul, Bl>>>>;
help: you might be missing a type parameter
     |
     |
1735 | impl<Ul: Unsigned, Bl: Bit, Ur: Unsigned, Br: Bit, U0> Rem<UInt<Ur, Br>> for UInt<Ul, Bl>

error[E0412]: cannot find type `U0` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1754:38
     |
     |
1754 | impl<N, D, Q, I> PrivateDiv<N, D, Q, U0, I> for ()
     | 
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:21:1
     |
21   | pub struct B0;
21   | pub struct B0;
     | -------------- similarly named struct `B0` defined here
     |
help: a struct with a similar name exists
     |
1754 | impl<N, D, Q, I> PrivateDiv<N, D, Q, B0, I> for ()
help: you might be missing a type parameter
     |
     |
1754 | impl<N, D, Q, I, U0> PrivateDiv<N, D, Q, U0, I> for ()

error[E0412]: cannot find type `U0` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1786:56
     |
     |
1786 |     fn private_div_quotient(self, n: N, d: D, q: Q, _: U0, i: I) -> Self::Quotient
     |                                                        ^^ help: a struct with a similar name exists: `B0`
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:21:1
     |
21   | pub struct B0;
     | -------------- similarly named struct `B0` defined here
     | -------------- similarly named struct `B0` defined here

error[E0412]: cannot find type `U0` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1798:57
     |
1798 |     fn private_div_remainder(self, n: N, d: D, q: Q, _: U0, i: I) -> Self::Remainder {
     |                                                         ^^ help: a struct with a similar name exists: `B0`
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:21:1
     |
21   | pub struct B0;
     | -------------- similarly named struct `B0` defined here
     | -------------- similarly named struct `B0` defined here

error[E0412]: cannot find type `U0` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1909:58
     |
1909 |     (): PrivateDiv<N, D, SetBitOut<Q, UInt<Ui, Bi>, B1>, U0, Sub1<UInt<Ui, Bi>>>,
     | 
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:21:1
     |
21   | pub struct B0;
21   | pub struct B0;
     | -------------- similarly named struct `B0` defined here
     |
help: a struct with a similar name exists
     |
1909 |     (): PrivateDiv<N, D, SetBitOut<Q, UInt<Ui, Bi>, B1>, B0, Sub1<UInt<Ui, Bi>>>,
help: you might be missing a type parameter
     |
     |
1905 | impl<N, D, Q, R, Ui, Bi, U0> PrivateDivIf<N, D, Q, R, UInt<Ui, Bi>, Equal> for ()

error[E0412]: cannot find type `U0` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1911:74
     |
     |
1911 |     type Quotient = PrivateDivQuot<N, D, SetBitOut<Q, UInt<Ui, Bi>, B1>, U0, Sub1<UInt<Ui, Bi>>>;
     | 
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:21:1
     |
21   | pub struct B0;
21   | pub struct B0;
     | -------------- similarly named struct `B0` defined here
     |
help: a struct with a similar name exists
     |
1911 |     type Quotient = PrivateDivQuot<N, D, SetBitOut<Q, UInt<Ui, Bi>, B1>, B0, Sub1<UInt<Ui, Bi>>>;
help: you might be missing a type parameter
     |
     |
1905 | impl<N, D, Q, R, Ui, Bi, U0> PrivateDivIf<N, D, Q, R, UInt<Ui, Bi>, Equal> for ()

error[E0412]: cannot find type `U0` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1912:74
     |
     |
1912 |     type Remainder = PrivateDivRem<N, D, SetBitOut<Q, UInt<Ui, Bi>, B1>, U0, Sub1<UInt<Ui, Bi>>>;
     | 
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:21:1
     |
21   | pub struct B0;
21   | pub struct B0;
     | -------------- similarly named struct `B0` defined here
     |
help: a struct with a similar name exists
     |
1912 |     type Remainder = PrivateDivRem<N, D, SetBitOut<Q, UInt<Ui, Bi>, B1>, B0, Sub1<UInt<Ui, Bi>>>;
help: you might be missing a type parameter
     |
     |
1905 | impl<N, D, Q, R, Ui, Bi, U0> PrivateDivIf<N, D, Q, R, UInt<Ui, Bi>, Equal> for ()

error[E0412]: cannot find type `U0` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1988:43
     |
     |
1988 | impl<N, D, Q, R> PrivateDivIf<N, D, Q, R, U0, Less> for () {
     | 
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:21:1
     |
21   | pub struct B0;
21   | pub struct B0;
     | -------------- similarly named struct `B0` defined here
     |
help: a struct with a similar name exists
     |
1988 | impl<N, D, Q, R> PrivateDivIf<N, D, Q, R, B0, Less> for () {
help: you might be missing a type parameter
     |
     |
1988 | impl<N, D, Q, R, U0> PrivateDivIf<N, D, Q, R, U0, Less> for () {

error[E0412]: cannot find type `U0` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1993:65
     |
     |
1993 |     fn private_div_if_quotient(self, _: N, _: D, q: Q, _: R, _: U0, _: Less) -> Self::Quotient {
     |                                                                 ^^ help: a struct with a similar name exists: `B0`
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:21:1
     |
21   | pub struct B0;
     | -------------- similarly named struct `B0` defined here
     | -------------- similarly named struct `B0` defined here

error[E0412]: cannot find type `U0` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:1998:66
     |
1998 |     fn private_div_if_remainder(self, _: N, _: D, _: Q, r: R, _: U0, _: Less) -> Self::Remainder {
     |                                                                  ^^ help: a struct with a similar name exists: `B0`
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:21:1
     |
21   | pub struct B0;
     | -------------- similarly named struct `B0` defined here
     | -------------- similarly named struct `B0` defined here

error[E0412]: cannot find type `U0` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:2004:43
     |
2004 | impl<N, D, Q, R> PrivateDivIf<N, D, Q, R, U0, Equal> for ()
     | 
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:21:1
     |
21   | pub struct B0;
21   | pub struct B0;
     | -------------- similarly named struct `B0` defined here
     |
help: a struct with a similar name exists
     |
2004 | impl<N, D, Q, R> PrivateDivIf<N, D, Q, R, B0, Equal> for ()
help: you might be missing a type parameter
     |
     |
2004 | impl<N, D, Q, R, U0> PrivateDivIf<N, D, Q, R, U0, Equal> for ()

error[E0412]: cannot find type `U0` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:2006:15
     |
     |
2006 |     Q: SetBit<U0, B1>,
     | 
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:21:1
     |
21   | pub struct B0;
21   | pub struct B0;
     | -------------- similarly named struct `B0` defined here
     |
help: a struct with a similar name exists
     |
2006 |     Q: SetBit<B0, B1>,
help: you might be missing a type parameter
     |
     |
2004 | impl<N, D, Q, R, U0> PrivateDivIf<N, D, Q, R, U0, Equal> for ()

error[E0412]: cannot find type `U0` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:2008:34
     |
     |
2008 |     type Quotient = SetBitOut<Q, U0, B1>;
     | 
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:21:1
     |
21   | pub struct B0;
21   | pub struct B0;
     | -------------- similarly named struct `B0` defined here
     |
help: a struct with a similar name exists
     |
2008 |     type Quotient = SetBitOut<Q, B0, B1>;
help: you might be missing a type parameter
     |
     |
2004 | impl<N, D, Q, R, U0> PrivateDivIf<N, D, Q, R, U0, Equal> for ()

error[E0412]: cannot find type `U0` in this scope
---

error[E0412]: cannot find type `U0` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:2012:65
     |
2012 |     fn private_div_if_quotient(self, _: N, _: D, q: Q, _: R, i: U0, _: Equal) -> Self::Quotient {
     |                                                                 ^^ help: a struct with a similar name exists: `B0`
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:21:1
     |
21   | pub struct B0;
     | -------------- similarly named struct `B0` defined here
     | -------------- similarly named struct `B0` defined here

error[E0412]: cannot find type `U0` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:2017:66
     |
2017 |     fn private_div_if_remainder(self, _: N, _: D, _: Q, _: R, i: U0, _: Equal) -> Self::Remainder {
     |                                                                  ^^ help: a struct with a similar name exists: `B0`
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:21:1
     |
21   | pub struct B0;
     | -------------- similarly named struct `B0` defined here
     | -------------- similarly named struct `B0` defined here

error[E0412]: cannot find type `U0` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:2023:43
     |
2023 | impl<N, D, Q, R> PrivateDivIf<N, D, Q, R, U0, Greater> for ()
     | 
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:21:1
     |
21   | pub struct B0;
21   | pub struct B0;
     | -------------- similarly named struct `B0` defined here
     |
help: a struct with a similar name exists
     |
2023 | impl<N, D, Q, R> PrivateDivIf<N, D, Q, R, B0, Greater> for ()
help: you might be missing a type parameter
     |
     |
2023 | impl<N, D, Q, R, U0> PrivateDivIf<N, D, Q, R, U0, Greater> for ()

error[E0412]: cannot find type `U0` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:2026:15
     |
     |
2026 |     Q: SetBit<U0, B1>,
     | 
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:21:1
     |
21   | pub struct B0;
21   | pub struct B0;
     | -------------- similarly named struct `B0` defined here
     |
help: a struct with a similar name exists
     |
2026 |     Q: SetBit<B0, B1>,
help: you might be missing a type parameter
     |
     |
2023 | impl<N, D, Q, R, U0> PrivateDivIf<N, D, Q, R, U0, Greater> for ()

error[E0412]: cannot find type `U0` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:2028:34
     |
     |
2028 |     type Quotient = SetBitOut<Q, U0, B1>;
     | 
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:21:1
     |
21   | pub struct B0;
21   | pub struct B0;
     | -------------- similarly named struct `B0` defined here
     |
help: a struct with a similar name exists
     |
2028 |     type Quotient = SetBitOut<Q, B0, B1>;
help: you might be missing a type parameter
     |
     |
2023 | impl<N, D, Q, R, U0> PrivateDivIf<N, D, Q, R, U0, Greater> for ()

error[E0412]: cannot find type `U0` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:2032:65
     |
     |
2032 |     fn private_div_if_quotient(self, _: N, _: D, q: Q, _: R, i: U0, _: Greater) -> Self::Quotient {
     |                                                                 ^^ help: a struct with a similar name exists: `B0`
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:21:1
     |
21   | pub struct B0;
     | -------------- similarly named struct `B0` defined here
---

error[E0412]: cannot find type `U0` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/uint.rs:2064:66
     |
2064 |     UInt<Ul, Bl>: Div<UInt<Ur, Br>> + Rem<UInt<Ur, Br>, Output = U0>,
     |                                                                  ^^ help: a struct with a similar name exists: `B0`
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/bit.rs:21:1
     |
21   | pub struct B0;
     | -------------- similarly named struct `B0` defined here
---

error[E0412]: cannot find type `B1` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/array.rs:66:20
   |
66 |     Length<A>: Add<B1>,
   |                    ^^ not found in this scope
help: consider importing this struct
   |
5  | use bit::B1;
   |
   |

error[E0412]: cannot find type `B1` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/array.rs:67:20
   |
67 |     Sum<Length<A>, B1>: Unsigned,
   |                    ^^ not found in this scope
help: consider importing this struct
   |
5  | use bit::B1;
   |
   |

error[E0425]: cannot find value `B1` in this scope
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/array.rs:72:27
   |
72 |         self.rest.len() + B1
   |                           ^^ not found in this scope
help: consider importing this unit struct
   |
5  | use bit::B1;
   |
   |

error[E0412]: cannot find type `Z0` in this scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/array.rs:157:21
    |
157 | impl Mul<ATerm> for Z0 {
    |                     ^^ not found in this scope
help: consider importing this struct
    |
5   | use int::Z0;
    |
    |

error[E0412]: cannot find type `Z0` in this scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/array.rs:187:32
    |
187 | impl<V, A> Mul<TArr<V, A>> for Z0
    |                                ^^ not found in this scope
help: consider importing this struct
    |
5   | use int::Z0;
    |
    |

error[E0412]: cannot find type `Z0` in this scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/array.rs:189:5
    |
189 |     Z0: Mul<A>,
    |     ^^ not found in this scope
help: consider importing this struct
    |
5   | use int::Z0;
    |
    |

error[E0412]: cannot find type `Z0` in this scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/array.rs:191:24
    |
191 |     type Output = TArr<Z0, Prod<Z0, A>>;
    |                        ^^ not found in this scope
help: consider importing this struct
    |
5   | use int::Z0;
    |
    |

error[E0412]: cannot find type `Z0` in this scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/array.rs:191:33
    |
191 |     type Output = TArr<Z0, Prod<Z0, A>>;
    |                                 ^^ not found in this scope
help: consider importing this struct
    |
5   | use int::Z0;
    |
    |

error[E0425]: cannot find value `Z0` in this scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/array.rs:195:20
195 |             first: Z0,
    |                    ^^ not found in this scope
    |
help: consider importing this unit struct
help: consider importing this unit struct
    |
5   | use int::Z0;
    |

error[E0282]: type annotations needed
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/array.rs:63:6
   |
63 | impl<V, A> Len for TArr<V, A>
   |      ^ cannot infer type for type parameter `V`
error[E0282]: type annotations needed
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.12.0/src/array.rs:71:5
   |
   |
71 | /     fn len(&self) -> Self::Output {
72 | |         self.rest.len() + B1
   | |_____^ cannot infer type for type parameter `V`

   Compiling psm v0.1.11
error: aborting due to 106 previous errors
