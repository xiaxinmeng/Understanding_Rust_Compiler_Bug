plain
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.91
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0119]: conflicting implementations of trait `PartialEq` for type `&_`
     |
     |
1440 |     impl<A: ?Sized, B: ?Sized> PartialEq<&B> for &A
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `&_`
    ::: library/core/src/ptr/mod.rs:1888:1
     |
     |
1888 | impl<F: FnPtr> PartialEq for F {
     |
     |
     = note: downstream crates may implement trait `marker::FnPtr` for type `&_`

error[E0119]: conflicting implementations of trait `PartialEq` for type `&mut _`
     |
     |
1495 |     impl<A: ?Sized, B: ?Sized> PartialEq<&mut B> for &mut A
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `&mut _`
    ::: library/core/src/ptr/mod.rs:1888:1
     |
     |
1888 | impl<F: FnPtr> PartialEq for F {
     |
     |
     = note: downstream crates may implement trait `marker::FnPtr` for type `&mut _`

error[E0119]: conflicting implementations of trait `PartialEq` for type `Pin<_>`
     |
     |
426  | impl<P: Deref, Q: Deref> PartialEq<Pin<Q>> for Pin<P>
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `Pin<_>`
    ::: library/core/src/ptr/mod.rs:1888:1
     |
     |
1888 | impl<F: FnPtr> PartialEq for F {
     |
     |
     = note: downstream crates may implement trait `marker::FnPtr` for type `pin::Pin<_>`

error[E0119]: conflicting implementations of trait `Eq` for type `&_`
     |
     |
1490 |     impl<A: ?Sized> Eq for &A where A: Eq {}
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `&_`
    ::: library/core/src/ptr/mod.rs:1895:1
     |
     |
1895 | impl<F: FnPtr> Eq for F {}
     |
     |
     = note: downstream crates may implement trait `marker::FnPtr` for type `&_`

error[E0119]: conflicting implementations of trait `Eq` for type `&mut _`
     |
     |
1545 |     impl<A: ?Sized> Eq for &mut A where A: Eq {}
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `&mut _`
    ::: library/core/src/ptr/mod.rs:1895:1
     |
     |
1895 | impl<F: FnPtr> Eq for F {}
     |
     |
     = note: downstream crates may implement trait `marker::FnPtr` for type `&mut _`

error[E0119]: conflicting implementations of trait `Eq` for type `Pin<_>`
     |
     |
440  | impl<P: Deref<Target: Eq>> Eq for Pin<P> {}
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `Pin<_>`
    ::: library/core/src/ptr/mod.rs:1895:1
     |
     |
1895 | impl<F: FnPtr> Eq for F {}
     |
     |
     = note: downstream crates may implement trait `marker::FnPtr` for type `pin::Pin<_>`

error[E0119]: conflicting implementations of trait `fmt::Debug` for type `Pin<_>`
     |
     |
975  | impl<P: fmt::Debug> fmt::Debug for Pin<P> {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `Pin<_>`
    ::: library/core/src/ptr/mod.rs:1927:1
     |
     |
1927 | impl<F: FnPtr> fmt::Debug for F {
     |
     |
     = note: downstream crates may implement trait `marker::FnPtr` for type `pin::Pin<_>`

error[E0119]: conflicting implementations of trait `fmt::Debug` for type `&_`
     |
2253 | / macro_rules! fmt_refs {
2253 | / macro_rules! fmt_refs {
2254 | |     ($($tr:ident),*) => {
2256 | |         #[stable(feature = "rust1", since = "1.0.0")]
2256 | |         #[stable(feature = "rust1", since = "1.0.0")]
2257 | |         impl<T: ?Sized + $tr> $tr for &T {
     | |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `&_`
2265 | |     }
2266 | | }
2266 | | }
     | |_- in this expansion of `fmt_refs!`
2267 |
2268 |   fmt_refs! { Debug, Display, Octal, Binary, LowerHex, UpperHex, LowerExp, UpperExp }
     |
    ::: library/core/src/ptr/mod.rs:1927:1
     |
     |
1927 |   impl<F: FnPtr> fmt::Debug for F {
     |
     |
     = note: downstream crates may implement trait `marker::FnPtr` for type `&_`

error[E0119]: conflicting implementations of trait `fmt::Debug` for type `&mut _`
     |
2253 | / macro_rules! fmt_refs {
2253 | / macro_rules! fmt_refs {
2254 | |     ($($tr:ident),*) => {
2256 | |         #[stable(feature = "rust1", since = "1.0.0")]
...    |
...    |
2261 | |         impl<T: ?Sized + $tr> $tr for &mut T {
     | |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `&mut _`
2265 | |     }
2266 | | }
2266 | | }
     | |_- in this expansion of `fmt_refs!`
2267 |
2268 |   fmt_refs! { Debug, Display, Octal, Binary, LowerHex, UpperHex, LowerExp, UpperExp }
     |
    ::: library/core/src/ptr/mod.rs:1927:1
     |
     |
1927 |   impl<F: FnPtr> fmt::Debug for F {
     |
     |
     = note: downstream crates may implement trait `marker::FnPtr` for type `&mut _`

error[E0119]: conflicting implementations of trait `PartialOrd` for type `&_`
     |
     |
1454 |     impl<A: ?Sized, B: ?Sized> PartialOrd<&B> for &A
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `&_`
    ::: library/core/src/ptr/mod.rs:1898:1
     |
     |
1898 | impl<F: FnPtr> PartialOrd for F {
     |
     |
     = note: downstream crates may implement trait `marker::FnPtr` for type `&_`
error[E0119]: conflicting implementations of trait `PartialOrd` for type `&mut _`
    --> library/core/src/cmp.rs:1509:5
     |
     |
1509 |     impl<A: ?Sized, B: ?Sized> PartialOrd<&mut B> for &mut A
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `&mut _`
    ::: library/core/src/ptr/mod.rs:1898:1
     |
     |
1898 | impl<F: FnPtr> PartialOrd for F {
     |
     |
     = note: downstream crates may implement trait `marker::FnPtr` for type `&mut _`

error[E0119]: conflicting implementations of trait `PartialOrd` for type `Pin<_>`
     |
     |
443  | impl<P: Deref, Q: Deref> PartialOrd<Pin<Q>> for Pin<P>
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `Pin<_>`
    ::: library/core/src/ptr/mod.rs:1898:1
     |
     |
1898 | impl<F: FnPtr> PartialOrd for F {
     |
     |
     = note: downstream crates may implement trait `marker::FnPtr` for type `pin::Pin<_>`

error[E0119]: conflicting implementations of trait `Ord` for type `&_`
     |
     |
1480 |     impl<A: ?Sized> Ord for &A
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `&_`
    ::: library/core/src/ptr/mod.rs:1905:1
     |
     |
1905 | impl<F: FnPtr> Ord for F {
     |
     |
     = note: downstream crates may implement trait `marker::FnPtr` for type `&_`

error[E0119]: conflicting implementations of trait `Ord` for type `&mut _`
     |
     |
1535 |     impl<A: ?Sized> Ord for &mut A
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `&mut _`
    ::: library/core/src/ptr/mod.rs:1905:1
     |
     |
1905 | impl<F: FnPtr> Ord for F {
     |
     |
     = note: downstream crates may implement trait `marker::FnPtr` for type `&mut _`

error[E0119]: conflicting implementations of trait `Ord` for type `Pin<_>`
     |
     |
469  | impl<P: Deref<Target: Ord>> Ord for Pin<P> {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `Pin<_>`
    ::: library/core/src/ptr/mod.rs:1905:1
     |
     |
1905 | impl<F: FnPtr> Ord for F {
     |
     |
     = note: downstream crates may implement trait `marker::FnPtr` for type `pin::Pin<_>`

error[E0119]: conflicting implementations of trait `hash::Hash` for type `Pin<_>`
     |
     |
476  | impl<P: Deref<Target: Hash>> Hash for Pin<P> {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `Pin<_>`
    ::: library/core/src/ptr/mod.rs:1913:1
     |
     |
1913 | impl<F: FnPtr> hash::Hash for F {
     |
     |
     = note: downstream crates may implement trait `marker::FnPtr` for type `pin::Pin<_>`

error[E0119]: conflicting implementations of trait `hash::Hash` for type `&_`
     |
     |
944  |     impl<T: ?Sized + Hash> Hash for &T {
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `&_`
    ::: library/core/src/ptr/mod.rs:1913:1
     |
     |
1913 | impl<F: FnPtr> hash::Hash for F {
     |
     |
     = note: downstream crates may implement trait `marker::FnPtr` for type `&_`

error[E0119]: conflicting implementations of trait `hash::Hash` for type `&mut _`
     |
     |
952  |     impl<T: ?Sized + Hash> Hash for &mut T {
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `&mut _`
    ::: library/core/src/ptr/mod.rs:1913:1
     |
     |
1913 | impl<F: FnPtr> hash::Hash for F {
     |
     |
     = note: downstream crates may implement trait `marker::FnPtr` for type `&mut _`

error[E0119]: conflicting implementations of trait `fmt::Pointer` for type `Pin<_>`
     |
     |
989  | impl<P: fmt::Pointer> fmt::Pointer for Pin<P> {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `Pin<_>`
    ::: library/core/src/ptr/mod.rs:1920:1
     |
     |
1920 | impl<F: FnPtr> fmt::Pointer for F {
     |
     |
     = note: downstream crates may implement trait `marker::FnPtr` for type `pin::Pin<_>`

error[E0119]: conflicting implementations of trait `fmt::Pointer` for type `&_`
     |
     |
2405 | impl<T: ?Sized> Pointer for &T {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `&_`
    ::: library/core/src/ptr/mod.rs:1920:1
     |
     |
1920 | impl<F: FnPtr> fmt::Pointer for F {
     |
     |
     = note: downstream crates may implement trait `marker::FnPtr` for type `&_`
error[E0119]: conflicting implementations of trait `fmt::Pointer` for type `&mut _`
    --> library/core/src/fmt/mod.rs:2412:1
     |
     |
2412 | impl<T: ?Sized> Pointer for &mut T {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `&mut _`
    ::: library/core/src/ptr/mod.rs:1920:1
     |
     |
1920 | impl<F: FnPtr> fmt::Pointer for F {
     |
     |
     = note: downstream crates may implement trait `marker::FnPtr` for type `&mut _`
For more information about this error, try `rustc --explain E0119`.
error: could not compile `core` (lib) due to 21 previous errors
Build completed unsuccessfully in 0:03:24
